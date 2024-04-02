use std::{
    io::Read,
    net::{Ipv4Addr, SocketAddr},
    path::{Path, PathBuf},
    thread,
    time::Duration,
};

use crate::{common::CommunicationProtocol, PacketExt};
use anyhow::{Context, Result};
use bytes::{Bytes, BytesMut};
use crossbeam_channel::Receiver;
use rtp::packet::Packet;
use tracing::{error, info};
use video_compositor::config::read_config;
use webrtc_util::Unmarshal;

pub struct OutputReceiver {
    receiver: Receiver<Bytes>,
    dump_length: Duration,
}

impl OutputReceiver {
    pub fn start<P: AsRef<Path>>(
        port: u16,
        protocol: CommunicationProtocol,
        dump_length: Duration,
        dump_path: P,
    ) -> Result<Self> {
        let config = read_config();
        let mut socket = Self::setup_socket(port, &protocol)?;

        let mut start_pts = None;
        let mut output_dump = BytesMut::new();
        let dump_path = dump_path.as_ref().to_path_buf();
        let (dump_sender, dump_receiver) = crossbeam_channel::bounded(1);

        thread::spawn(move || {
            loop {
                let (packet, packet_bytes) = match Self::read_packet(&mut socket, &protocol) {
                    Ok(packet) => packet,
                    Err(err) => {
                        error!("Failed to read packet: {err}");
                        break;
                    }
                };

                output_dump.extend_from_slice(&packet_bytes);
                let start_pts = start_pts.get_or_insert_with(|| packet.pts(&config));
                if packet.pts(&config) - *start_pts >= dump_length {
                    break;
                }
            }

            if cfg!(feature = "update_snapshots") {
                info!("Updating output dump: {dump_path:?}");
                let save_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .parent()
                    .unwrap()
                    .join("snapshot_tests")
                    .join("snapshots")
                    .join("rtp_packet_dumps")
                    .join("outputs")
                    .join(dump_path);
                std::fs::write(save_path, &output_dump)
                    .context("Failed to write output dump")
                    .unwrap();
            }

            dump_sender.send(output_dump.freeze()).unwrap();
        });

        Ok(Self {
            receiver: dump_receiver,
            dump_length,
        })
    }

    pub fn wait_for_output(self) -> Result<Bytes> {
        self.receiver
            .recv_timeout(self.dump_length + Duration::from_secs(60))
            .context("Failed to receive output dump")
    }

    fn setup_socket(port: u16, protocol: &CommunicationProtocol) -> Result<socket2::Socket> {
        let socket = match protocol {
            CommunicationProtocol::Udp => socket2::Socket::new(
                socket2::Domain::IPV4,
                socket2::Type::DGRAM,
                Some(socket2::Protocol::UDP),
            )?,
            CommunicationProtocol::Tcp => socket2::Socket::new(
                socket2::Domain::IPV4,
                socket2::Type::STREAM,
                Some(socket2::Protocol::TCP),
            )?,
        };

        match protocol {
            CommunicationProtocol::Udp => {
                socket.bind(&SocketAddr::new(Ipv4Addr::new(127, 0, 0, 1).into(), port).into())?;
            }
            CommunicationProtocol::Tcp => {
                socket
                    .connect(&SocketAddr::new(Ipv4Addr::new(127, 0, 0, 1).into(), port).into())?;
            }
        }

        Ok(socket)
    }

    fn read_packet(
        socket: &mut socket2::Socket,
        protocol: &CommunicationProtocol,
    ) -> Result<(Packet, Bytes)> {
        match protocol {
            CommunicationProtocol::Udp => {
                let mut buffer = vec![0u8; u16::MAX as usize];
                let packet_len = socket.read(&mut buffer)?;
                let packet_len_bytes = (packet_len as u16).to_be_bytes();

                let packet = Packet::unmarshal(&mut &buffer[..packet_len]).unwrap();

                let mut packet_bytes = BytesMut::new();
                packet_bytes.extend(&packet_len_bytes);
                packet_bytes.extend(&buffer[..packet_len]);

                Ok((packet, packet_bytes.freeze()))
            }
            CommunicationProtocol::Tcp => {
                let mut packet_len_bytes = [0u8; 2];
                socket.read_exact(&mut packet_len_bytes)?;
                let packet_len = u16::from_be_bytes(packet_len_bytes) as usize;

                let mut buffer = vec![0u8; packet_len];
                socket.read_exact(&mut buffer[..])?;

                let packet = Packet::unmarshal(&mut &buffer[..]).unwrap();

                let mut packet_bytes = BytesMut::new();
                packet_bytes.extend(&packet_len_bytes);
                packet_bytes.extend(&buffer[..packet_len]);

                Ok((packet, packet_bytes.freeze()))
            }
        }
    }
}
