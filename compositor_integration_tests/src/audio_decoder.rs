use std::time::Duration;

use anyhow::Result;

use rtp::{codecs::opus::OpusPacket, packetizer::Depacketizer};
use video_compositor::config::Config;

use crate::PacketExt;

#[derive(Clone)]
pub struct AudioSampleBatch {
    pub samples: Vec<i16>,
    pub pts: Duration,
}

#[derive(Debug, Clone, Copy)]
pub enum AudioChannels {
    Mono,
    Stereo,
}

pub struct AudioDecoder {
    buffer: Vec<i16>,
    depayloader: OpusPacket,
    decoder: opus::Decoder,
    decoded_samples: Vec<AudioSampleBatch>,
    config: Config,
}

impl AudioDecoder {
    pub fn new(config: Config, channels: AudioChannels) -> Result<Self> {
        let channels = match channels {
            AudioChannels::Mono => opus::Channels::Mono,
            AudioChannels::Stereo => opus::Channels::Stereo,
        };
        let decoder = opus::Decoder::new(config.output_sample_rate, channels)?;

        Ok(Self {
            buffer: vec![0; config.output_sample_rate as usize * 20],
            depayloader: OpusPacket,
            decoder,
            decoded_samples: Vec::new(),
            config,
        })
    }

    pub fn decode(&mut self, packet: rtp::packet::Packet) -> Result<()> {
        let chunk_data = self.depayloader.depacketize(&packet.payload)?;
        if chunk_data.is_empty() {
            return Ok(());
        }

        let samples_count = self.decoder.decode(&chunk_data, &mut self.buffer, false)?;
        self.decoded_samples.push(AudioSampleBatch {
            samples: self.buffer[..samples_count].to_vec(),
            pts: packet.pts(&self.config),
        });

        Ok(())
    }

    pub fn take_samples(self) -> Vec<AudioSampleBatch> {
        self.decoded_samples
    }
}
