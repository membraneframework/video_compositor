use compositor_render::InputId;
use tracing::error;

use self::{capture::ChannelCallbackAdapter, connect_device::init_decklink};

use super::{AudioInputReceiver, Input, InputInitInfo, InputInitResult, VideoInputReceiver};

mod capture;
mod connect_device;

#[derive(Debug, thiserror::Error)]
pub enum DeckLinkError {
    #[error(transparent)]
    DecklinkError(#[from] decklink::DeckLinkError),
    #[error("DeckLink device with capture support was not detected.")]
    DeckLinkWithCaptureNotFound,
}

pub struct DeckLinkOptions {
    pub device: DeckLinkSelectMode,
}

pub enum DeckLinkSelectMode {
    Auto,
    SubDeviceIndex(u32),
}

pub struct DeckLink {
    input: decklink::Input,
}

impl DeckLink {
    pub(super) fn start_new_input(
        input_id: &InputId,
        opts: DeckLinkOptions,
    ) -> Result<InputInitResult, DeckLinkError> {
        let mut input = init_decklink()?;

        let (callback, video_receiver, audio_receiver) = ChannelCallbackAdapter::new();
        input.set_callback(Box::new(callback))?;
        input.start_streams()?;

        Ok(InputInitResult {
            input: Input::DeckLink(Self { input }),
            video: video_receiver.map(|rec| VideoInputReceiver::Raw {
                frame_receiver: rec,
            }),
            audio: audio_receiver.map(|rec| AudioInputReceiver::Raw {
                sample_receiver: rec,
                sample_rate: 48000,
            }),
            init_info: InputInitInfo { port: None },
        })
    }
}

impl Drop for DeckLink {
    fn drop(&mut self) {
        if let Err(err) = self.input.stop_streams() {
            error!("Failed to stop streams: {:?}", err);
        }
    }
}
