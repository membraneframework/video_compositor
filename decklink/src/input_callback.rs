use crate::api::{AudioInputPacket, VideoInputFrame};

pub enum InputCallbackResult {
    Ok,
    Failure,
}

pub trait InputCallback {
    fn video_input_frame_arrived(
        &self,
        video_frame: Option<&mut VideoInputFrame>,
        audio_packet: Option<&mut AudioInputPacket>,
    ) -> InputCallbackResult;
}
