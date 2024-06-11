use compositor_render::Frame;
use crossbeam_channel::{bounded, Receiver, Sender};
use decklink::{AudioInputPacket, InputCallback, InputCallbackResult, VideoInputFrame};

use crate::{pipeline::structs::DecodedSamples, queue::PipelineEvent};

pub(super) struct ChannelCallbackAdapter {
    video_sender: Option<Sender<PipelineEvent<Frame>>>,
    audio_sender: Option<Sender<PipelineEvent<DecodedSamples>>>,
}

impl ChannelCallbackAdapter {
    pub(super) fn new() -> (
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
            },
            Some(video_receiver),
            Some(audio_receiver),
        )
    }
}

impl InputCallback for ChannelCallbackAdapter {
    fn video_input_frame_arrived(
        &self,
        video_frame: Option<&mut VideoInputFrame>,
        audio_packet: Option<&mut AudioInputPacket>,
    ) -> InputCallbackResult {
        InputCallbackResult::Ok
    }
}
