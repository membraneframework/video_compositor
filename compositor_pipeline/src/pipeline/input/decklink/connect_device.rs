use decklink::{
    find_decklink_with_capture_support, AudioSampleType, DisplayMode, PixelFormat,
    SupportedVideoModeFlags, VideoConnection, VideoInputConversionMode, VideoInputFlags,
};

use super::DeckLinkError;

pub(super) fn init_decklink() -> Result<decklink::Input, DeckLinkError> {
    let Some(decklink) = find_decklink_with_capture_support()? else {
        return Err(DeckLinkError::DeckLinkWithCaptureNotFound);
    };
    let mut input = decklink.input()?;
    let mode = input.supports_video_mode(
        VideoConnection::HDMI,
        DisplayMode::ModeUnknown,
        PixelFormat::Format8BitYUV,
        VideoInputConversionMode::NoVideoInputConversion,
        SupportedVideoModeFlags::default(),
    )?;
    input.enable_video_output(mode, PixelFormat::Format8BitYUV, VideoInputFlags::default())?;
    input.enable_audio_output(48_000, AudioSampleType::Sample32bit, 2)?;

    return Ok(input);
}
