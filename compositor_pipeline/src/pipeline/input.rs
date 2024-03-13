use std::path::Path;

use crate::{error::InputInitError, queue::PipelineEvent};

use crossbeam_channel::Receiver;
use rtp::{RtpReceiver, RtpReceiverOptions};

use self::{
    hls::{Hls, HlsOptions},
    mp4::{Mp4, Mp4Options},
};

use super::{decoder::DecoderOptions, structs::EncodedChunk, Port};

pub mod hls;
pub mod mp4;
pub mod rtp;

pub enum Input {
    Rtp(RtpReceiver),
    Mp4(Mp4),
    Hls(Hls),
}

impl Input {
    pub fn new(
        options: InputOptions,
        download_dir: &Path,
    ) -> Result<(Self, ChunksReceiver, DecoderOptions, Option<Port>), InputInitError> {
        match options {
            InputOptions::Rtp(opts) => Ok(RtpReceiver::new(opts).map(
                |(receiver, chunks_receiver, decoder_options, port)| {
                    (
                        Self::Rtp(receiver),
                        chunks_receiver,
                        decoder_options,
                        Some(port),
                    )
                },
            )?),

            InputOptions::Mp4(opts) => Ok(Mp4::new(opts, download_dir).map(
                |(mp4, chunks_receiver, decoder_options)| {
                    (Self::Mp4(mp4), chunks_receiver, decoder_options, None)
                },
            )?),

            InputOptions::Hls(opts) => Ok(Hls::new(opts).map(
                |(hls, chunks_receiver, decoder_options)| {
                    (Self::Hls(hls), chunks_receiver, decoder_options, None)
                },
            )?),
        }
    }
}

pub enum InputOptions {
    Rtp(RtpReceiverOptions),
    Mp4(Mp4Options),
    Hls(HlsOptions),
}

#[derive(Debug)]
pub struct ChunksReceiver {
    pub video: Option<Receiver<PipelineEvent<EncodedChunk>>>,
    pub audio: Option<Receiver<PipelineEvent<EncodedChunk>>>,
}
