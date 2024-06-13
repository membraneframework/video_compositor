use compositor_render::InputId;
use tracing::{error, span, Level};

use self::{capture::ChannelCallbackAdapter, connect_device::init_decklink};

use super::{AudioInputReceiver, Input, InputInitInfo, InputInitResult, VideoInputReceiver};

mod capture;
mod connect_device;

const AUDIO_SAMPLE_RATE: u32 = 48_000;

#[derive(Debug, thiserror::Error)]
pub enum DeckLinkError {
    #[error("Unknown DeckLink error.")]
    DecklinkError(#[from] decklink::DeckLinkError),
    #[error("DeckLink device with capture support was not detected.")]
    DeckLinkWithCaptureNotFound,
}

pub struct DeckLinkOptions {
    pub device_id: Option<u32>,
    pub subdevice: Option<u32>,
}

pub struct DeckLink {
    input: decklink::Input,
}

impl DeckLink {
    pub(super) fn start_new_input(
        input_id: &InputId,
        opts: DeckLinkOptions,
    ) -> Result<InputInitResult, DeckLinkError> {
        let mut input = init_decklink(opts)?;

        let span = span!(
            Level::INFO,
            "DeckLink input",
            input_id = input_id.to_string()
        );
        let (callback, video_receiver, audio_receiver) = ChannelCallbackAdapter::new(span);
        input.set_callback(Box::new(callback))?;
        input.start_streams()?;

        Ok(InputInitResult {
            input: Input::DeckLink(Self { input }),
            video: video_receiver.map(|rec| VideoInputReceiver::Raw {
                frame_receiver: rec,
            }),
            audio: audio_receiver.map(|rec| AudioInputReceiver::Raw {
                sample_receiver: rec,
                sample_rate: AUDIO_SAMPLE_RATE,
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
