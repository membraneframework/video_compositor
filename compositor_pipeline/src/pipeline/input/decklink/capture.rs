use std::sync::Arc;

use bytes::{BufMut, BytesMut};
use compositor_render::{Frame, Resolution, YuvData, YuvVariant};
use crossbeam_channel::{bounded, Receiver, Sender};
use decklink::{
    AudioInputPacket, DetectedVideoInputFormatFlags, DisplayMode, InputCallback,
    InputCallbackResult, VideoInputFormatChangedEvents, VideoInputFrame,
};
use log::{debug, warn};
use tracing::Span;

use crate::{
    pipeline::structs::{DecodedSamples, Samples},
    queue::PipelineEvent,
};

use super::AUDIO_SAMPLE_RATE;

pub(super) struct ChannelCallbackAdapter {
    video_sender: Option<Sender<PipelineEvent<Frame>>>,
    audio_sender: Option<Sender<PipelineEvent<DecodedSamples>>>,
    span: Span,
}

impl ChannelCallbackAdapter {
    pub(super) fn new(
        span: Span,
    ) -> (
        Self,
        Option<Receiver<PipelineEvent<Frame>>>,
        Option<Receiver<PipelineEvent<DecodedSamples>>>,
    ) {
        let (video_sender, video_receiver) = bounded(1000);
        let (audio_sender, audio_receiver) = bounded(1000);
        (
            Self {
                video_sender: Some(video_sender),
                audio_sender: Some(audio_sender),
                span,
            },
            Some(video_receiver),
            Some(audio_receiver),
        )
    }

    fn handle_video_frame(
        &self,
        video_frame: &mut VideoInputFrame,
        sender: &Sender<PipelineEvent<Frame>>,
    ) {
        let width = video_frame.width();
        let height = video_frame.height();
        let bytes = video_frame.bytes().unwrap();
        let bytes_per_row = video_frame.bytes_per_row();
        let pts = video_frame.stream_time().unwrap();

        let mut y_plane = BytesMut::with_capacity(width * height);
        let mut u_plane = BytesMut::with_capacity((width / 2) * (height / 2));
        let mut v_plane = BytesMut::with_capacity((width / 2) * (height / 2));

        // TODO: temporary conversion in Rust. Rework it to do it on a GPU
        for (row_index, row) in bytes.chunks(bytes_per_row).enumerate() {
            for (index, pixel) in row.chunks(2).enumerate() {
                if index < width && pixel.len() >= 2 {
                    y_plane.put_u8(pixel[1]);
                }
            }
            if row_index % 2 == 0 && row_index * 2 < height {
                for (index, pixel) in row.chunks(2).enumerate() {
                    if index * 2 < width && pixel.len() >= 4 {
                        u_plane.put_u8(pixel[0]);
                        v_plane.put_u8(pixel[2]);
                    }
                }
            }
        }
        let frame = Frame {
            data: YuvData {
                variant: YuvVariant::YUV420P,
                y_plane: y_plane.freeze(),
                u_plane: u_plane.freeze(),
                v_plane: v_plane.freeze(),
            },
            resolution: Resolution { width, height },
            pts,
        };
        if sender.send(PipelineEvent::Data(frame)).is_err() {
            debug!("Failed to send frame from DeckLink. Channel closed.")
        }
    }

    fn handle_audio_packet(
        &self,
        audio_packet: &mut AudioInputPacket,
        sender: &Sender<PipelineEvent<DecodedSamples>>,
    ) {
        let pts = audio_packet.packet_time().unwrap();
        let samples = audio_packet.as_32_bit_stereo().unwrap();
        let samples = DecodedSamples {
            samples: Arc::new(Samples::Stereo32Bit(samples)),
            start_pts: pts,
            sample_rate: AUDIO_SAMPLE_RATE,
        };
        if sender.send(PipelineEvent::Data(samples)).is_err() {
            debug!("Failed to send samples from DeckLink. Channel closed.")
        }
    }
}

impl InputCallback for ChannelCallbackAdapter {
    fn video_input_frame_arrived(
        &self,
        video_frame: Option<&mut VideoInputFrame>,
        audio_packet: Option<&mut AudioInputPacket>,
    ) -> InputCallbackResult {
        let _span = self.span.enter();
        if let (Some(video_frame), Some(sender)) = (video_frame, &self.video_sender) {
            self.handle_video_frame(video_frame, sender)
        }
        if let (Some(audio_packet), Some(sender)) = (audio_packet, &self.audio_sender) {
            self.handle_audio_packet(audio_packet, sender)
        }
        InputCallbackResult::Ok
    }

    fn video_input_format_changed(
        &self,
        events: VideoInputFormatChangedEvents,
        _display_mode: DisplayMode,
        flags: DetectedVideoInputFormatFlags,
    ) -> InputCallbackResult {
        let _span = self.span.enter();
        warn!("Format changed {events:#?} {flags:#?}");
        InputCallbackResult::Ok
    }
}
