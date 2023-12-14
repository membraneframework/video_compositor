use std::thread;

use bytes::BytesMut;
use compositor_pipeline::{
    error::CustomError,
    pipeline::{decoder::DecoderParameters, PipelineInput},
};
use log::{error, warn};
use smol::{
    channel::{bounded, Receiver, Sender},
    future, net,
};
use webrtc_util::Unmarshal;

use crate::types::InputId;

pub struct RtpReceiver {
    receiver_thread: Option<thread::JoinHandle<()>>,
    should_close_tx: Sender<()>,
    pub port: u16,
}

pub struct Options {
    pub port: u16,
    pub input_id: InputId,
}

impl PipelineInput for RtpReceiver {
    type Opts = Options;
    type PacketIterator = PacketIter;

    fn new(opts: Self::Opts) -> Result<(Self, Self::PacketIterator), CustomError> {
        let (should_close_tx, should_close_rx) = bounded(1);
        let (packets_tx, packets_rx) = bounded(1024);

        // let socket = future::block_on(net::UdpSocket::bind(net::SocketAddrV4::new(
        //     net::Ipv4Addr::LOCALHOST,
        //     opts.port,
        // )))
        // .map_err(|e| CustomError(Box::new(e)))?;

        let socket = socket2::Socket::new(
            socket2::Domain::IPV4,
            socket2::Type::DGRAM,
            Some(socket2::Protocol::UDP),
        )
        .map_err(|e| CustomError(Box::new(e)))?;

        socket
            .set_recv_buffer_size(16 * 1024 * 1024)
            .map_err(|e| CustomError(Box::new(e)))?;
        
        socket
            .bind(&net::SocketAddr::V4(net::SocketAddrV4::new(net::Ipv4Addr::LOCALHOST, 0)).into())
            .map_err(|e| CustomError(Box::new(e)))?;

        let socket = std::net::UdpSocket::from(socket);

        let receiver_thread = thread::Builder::new()
            .name(format!("RTP receiver {}", opts.input_id))
            .spawn(move || {
                RtpReceiver::rtp_receiver_sync(socket, packets_tx, should_close_rx)
            })
            .unwrap();

        Ok((
            Self {
                port: opts.port,
                receiver_thread: Some(receiver_thread),
                should_close_tx,
            },
            PacketIter {
                receiver: packets_rx,
            },
        ))
    }

    fn decoder_parameters(&self) -> DecoderParameters {
        DecoderParameters {
            codec: compositor_pipeline::pipeline::decoder::Codec::H264,
        }
    }
}

impl RtpReceiver {
    fn rtp_receiver_sync(
        socket: std::net::UdpSocket,
        packets_tx: Sender<bytes::Bytes>,
        should_close_rx: Receiver<()>,
    ) {
        let mut buffer = BytesMut::zeroed(65536);
        socket
            .set_read_timeout(Some(std::time::Duration::from_millis(50)))
            .unwrap();

        loop {
            if should_close_rx.try_recv().is_ok() {
                return;
            }

            let received_bytes = match socket.recv(&mut buffer) {
                Ok(n) => n,
                Err(e) => match e.kind() {
                    std::io::ErrorKind::WouldBlock => continue,
                    _ => {
                        error!("Error while receiving UDP packet: {}", e);
                        continue;
                    }
                },
            };

            let packet: bytes::Bytes = buffer[..received_bytes].to_vec().into();
            packets_tx.send_blocking(packet).unwrap();
        }
    }
}

impl Drop for RtpReceiver {
    fn drop(&mut self) {
        self.should_close_tx.send_blocking(()).unwrap();
        if let Some(thread) = self.receiver_thread.take() {
            thread.join().unwrap();
        } else {
            error!("RTP receiver does not hold a thread handle to the receiving thread.")
        }
    }
}

pub struct PacketIter {
    receiver: Receiver<bytes::Bytes>,
}

impl Iterator for PacketIter {
    type Item = rtp::packet::Packet;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut buffer = self.receiver.recv_blocking().ok()?;

            match rtp::packet::Packet::unmarshal(&mut buffer.clone()) {
                // https://datatracker.ietf.org/doc/html/rfc5761#section-4
                //
                // Given these constraints, it is RECOMMENDED to follow the guidelines
                // in the RTP/AVP profile [7] for the choice of RTP payload type values,
                // with the additional restriction that payload type values in the range
                // 64-95 MUST NOT be used.
                Ok(packet)
                    if packet.header.payload_type < 64 || packet.header.payload_type > 95 =>
                {
                    return Some(packet);
                }
                Ok(_) | Err(_) => {
                    if rtcp::packet::unmarshal(&mut buffer).is_err() {
                        warn!("Received an unexpected packet, which is not recognized either as RTP or RTCP. Dropping.");
                    }

                    continue;
                }
            };
        }
    }
}
