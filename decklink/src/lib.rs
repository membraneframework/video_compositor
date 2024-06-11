use api::{get_decklinks, DeckLink};

pub(crate) mod api;
pub(crate) mod enums;
pub(crate) mod input_callback;

pub use enums::ffi::AudioSampleType;
pub use enums::ffi::DisplayMode;
pub use enums::ffi::PixelFormat;
pub use enums::ffi::SupportedVideoModeFlags;
pub use enums::ffi::VideoConnection;
pub use enums::ffi::VideoInputConversionMode;
pub use enums::ffi::VideoInputFlags;

pub use api::AudioInputPacket;
pub use api::Input;
pub use api::VideoInputFrame;
pub use input_callback::InputCallback;
pub use input_callback::InputCallbackResult;

//pub use input_callback::InputCallback;
//pub use input_callback::InputCallbackResult;
//
pub fn find_decklink_with_capture_support() -> Result<Option<DeckLink>, DeckLinkError> {
    let decklinks = get_decklinks()?;

    for decklink in decklinks {
        let attr = decklink.profile_attributes()?;
        let value = attr.get_integer(api::IntegerAttributeId::VideoIOSupport)?;
        let value = api::into_video_io_support(value);
        if value.capture {
            return Ok(Some(decklink));
        }
    }
    Ok(None)
}

#[derive(Debug, thiserror::Error)]
pub enum DeckLinkError {
    #[error("Unknown error: {0:#}")]
    UnknownError(#[from] cxx::Exception),
}
