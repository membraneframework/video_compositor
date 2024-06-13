use decklink::{
    get_decklinks, AudioSampleType, DisplayModeType, IntegerAttributeId, PixelFormat,
    SupportedVideoModeFlags, VideoConnection, VideoIOSupport, VideoInputConversionMode,
    VideoInputFlags,
};

use super::{DeckLinkError, DeckLinkOptions};

pub(super) fn init_decklink(opts: DeckLinkOptions) -> Result<decklink::Input, DeckLinkError> {
    let decklink = find_decklink(&opts)?;
    let mut input = decklink.input()?;
    let mode = input.supports_video_mode(
        VideoConnection::HDMI,
        DisplayModeType::ModeUnknown,
        PixelFormat::Format8BitYUV,
        VideoInputConversionMode::NoVideoInputConversion,
        SupportedVideoModeFlags::default(),
    )?;
    input.enable_video(mode, PixelFormat::Format8BitYUV, VideoInputFlags::default())?;
    input.enable_audio(48_000, AudioSampleType::Sample32bit, 2)?;

    return Ok(input);
}

fn find_decklink(opts: &DeckLinkOptions) -> Result<decklink::DeckLink, DeckLinkError> {
    let decklinks = get_decklinks()?;

    for mut decklink in decklinks.into_iter() {
        if is_selected_decklink(opts, &mut decklink)? {
            return Ok(decklink);
        }
    }
    Err(DeckLinkError::DeckLinkWithCaptureNotFound)
}

fn is_selected_decklink(
    opts: &DeckLinkOptions,
    decklink: &mut decklink::DeckLink,
) -> Result<bool, DeckLinkError> {
    let attr = decklink.profile_attributes()?;

    if let Some(subdevice) = opts.subdevice {
        if attr.get_integer(IntegerAttributeId::SubDeviceIndex)? as u32 != subdevice {
            return Ok(false);
        }
    }

    if let Some(device_id) = opts.device_id {
        if attr.get_integer(IntegerAttributeId::PersistentID)? as u32 != device_id {
            return Ok(false);
        }
    }

    let video_io_support =
        VideoIOSupport::from(attr.get_integer(IntegerAttributeId::VideoIOSupport)?);
    if !video_io_support.capture {
        return Ok(false);
    }

    return Ok(true);
}
