use std::sync::{atomic::AtomicBool, Arc};

use bytes::Bytes;
use compositor_render::InputId;
use crossbeam_channel::Sender;

use crate::{
    pipeline::{
        decoder::{AudioDecoderOptions, DecoderOptions, VideoDecoderOptions},
        input::mp4::{mp4_file_reader::Mp4FileReader, Mp4ReaderOptions},
    },
    queue::PipelineEvent,
};

use super::{mp4::Mp4Error, ChunksReceiver};

#[derive(Debug, thiserror::Error)]
pub enum HlsError {
    #[error("Request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("No compatible stream found")]
    NoStream,

    #[error("URL processing error: {0}")]
    UrlError(#[from] url::ParseError),

    #[error(transparent)]
    Mp4Error(#[from] Mp4Error),
}

pub struct HlsOptions {
    pub url: url::Url,
    pub buffer_length: std::time::Duration,
    pub input_id: InputId,
}

pub struct Hls {
    _audio_reader: Option<Mp4FileReader<AudioDecoderOptions>>,
    _video_reader: Option<Mp4FileReader<VideoDecoderOptions>>,
    stop_thread: Arc<AtomicBool>,
}

impl Hls {
    pub fn new(options: HlsOptions) -> Result<(Self, ChunksReceiver, DecoderOptions), HlsError> {
        // TODO: downloads shouldn't block the main thread
        let playlist = reqwest::blocking::get(options.url.clone())?.bytes()?;

        let playlist = m3u8_rs::parse_playlist_res(&playlist).unwrap();

        let playlist = match playlist {
            m3u8_rs::Playlist::MasterPlaylist(playlist) => {
                // TODO: better variant selection algorithm is necessary
                // TODO: support alternative streams and the `AUDIO` and `VIDEO` fields when selecting streams
                let stream = playlist
                    .variants
                    .iter()
                    .filter(|v| v.codecs.is_some() && v.codecs.as_ref().unwrap().contains("avc"))
                    .max_by_key(|v| v.bandwidth)
                    .ok_or(HlsError::NoStream)?;

                let url = options.url.clone().join(stream.uri.as_str())?;

                let playlist = reqwest::blocking::get(url)?.bytes()?;

                m3u8_rs::parse_media_playlist_res(&playlist).unwrap()
            }
            m3u8_rs::Playlist::MediaPlaylist(playlist) => playlist,
        };

        // TODO: this should happen when selecting the streams
        if playlist.segments.iter().any(|s| s.discontinuity) {
            return Err(HlsError::NoStream);
        }

        let header = options
            .url
            .clone()
            .join(&playlist.segments[0].map.as_ref().unwrap().uri)?;
        // TODO: support the `bytes` option
        let header = reqwest::blocking::get(header)?.bytes()?;

        let channel_len =
            (options.buffer_length.as_secs_f64() / playlist.target_duration as f64).ceil() as usize;

        let (video_sender, video_receiver) = crossbeam_channel::bounded(channel_len);
        let (audio_sender, audio_receiver) = crossbeam_channel::bounded(channel_len);

        let mut video_reader = None;
        let mut audio_reader = None;
        let mut chunks_receiver = ChunksReceiver {
            video: None,
            audio: None,
        };

        let mut decoder_options = DecoderOptions {
            video: None,
            audio: None,
        };

        let video_options = Mp4ReaderOptions::Fragmented {
            header: header.clone(),
            fragment_receiver: video_receiver,
        };

        if let Some((reader, receiver)) =
            Mp4FileReader::new_video(video_options, options.input_id.clone())?
        {
            chunks_receiver.video = Some(receiver);
            decoder_options.video = Some(reader.decoder_options());
            video_reader = Some(reader);
        }

        let audio_options = Mp4ReaderOptions::Fragmented {
            header: header.clone(),
            fragment_receiver: audio_receiver,
        };

        if let Some((reader, receiver)) =
            Mp4FileReader::new_audio(audio_options, options.input_id.clone())?
        {
            chunks_receiver.audio = Some(receiver);
            decoder_options.audio = Some(reader.decoder_options());
            audio_reader = Some(reader);
        }

        let stop_thread = Arc::new(AtomicBool::new(false));
        let stop_thread_clone = stop_thread.clone();

        let video_sender = video_reader.as_ref().map(move |_| video_sender);
        let audio_sender = audio_reader.as_ref().map(move |_| audio_sender);
        let senders = video_sender
            .into_iter()
            .chain(audio_sender)
            .collect::<Vec<_>>();

        println!("{playlist:#?}");

        std::thread::Builder::new()
            .name(format!("HLS downloader {}", options.input_id))
            .spawn(move || {
                run_downloading_thread(senders, stop_thread_clone, options.url, playlist)
            })
            .unwrap();

        Ok((
            Self {
                _audio_reader: audio_reader,
                _video_reader: video_reader,
                stop_thread,
            },
            chunks_receiver,
            decoder_options,
        ))
    }
}

impl Drop for Hls {
    fn drop(&mut self) {
        self.stop_thread
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }
}

fn run_downloading_thread(
    senders: Vec<Sender<PipelineEvent<Bytes>>>,
    _stop_thread: Arc<AtomicBool>,
    base_url: url::Url,
    media_playlist: m3u8_rs::MediaPlaylist,
) {
    for segment in media_playlist.segments {
        // TODO: unwraps
        let url = base_url.join(&segment.uri).unwrap();
        // TODO: support the `bytes` option
        let segment = reqwest::blocking::get(url).unwrap().bytes().unwrap();

        for sender in &senders {
            // TODO: timeout and check `stop_thread`
            sender.send(PipelineEvent::Data(segment.clone())).unwrap();
        }
    }
}
