pub(crate) mod api;
pub(crate) mod enums;
pub(crate) mod input_callback;

mod info;

use api::into_video_io_support;

pub use enums::ffi::FlagAttributeId;
pub use enums::ffi::FloatAttributeId;
pub use enums::ffi::IntegerAttributeId;
pub use enums::ffi::StringAttributeId;

pub use enums::ffi::AudioSampleType;
pub use enums::ffi::DetectedVideoInputFormatFlags;
pub use enums::ffi::DisplayModeType;
pub use enums::ffi::PixelFormat;
pub use enums::ffi::SupportedVideoModeFlags;
pub use enums::ffi::VideoConnection;
pub use enums::ffi::VideoInputConversionMode;
pub use enums::ffi::VideoInputFlags;
pub use enums::ffi::VideoInputFormatChangedEvents;

pub use api::input::AudioInputPacket;
pub use api::input::Input;
pub use api::input::VideoInputFrame;
pub use api::DeckLink;
pub use api::DisplayMode;
pub use api::VideoIOSupport;
pub use input_callback::InputCallback;
pub use input_callback::InputCallbackResult;

pub use api::get_decklinks;

pub fn find_decklink_with_capture_support() -> Result<Option<DeckLink>, DeckLinkError> {
    let decklinks = get_decklinks()?;

    for decklink in decklinks {
        let attr = decklink.profile_attributes()?;
        let value = attr.get_integer(enums::ffi::IntegerAttributeId::VideoIOSupport)?;
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

impl From<i64> for VideoIOSupport {
    fn from(value: i64) -> Self {
        into_video_io_support(value)
    }
}
