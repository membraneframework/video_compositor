use anyhow::{anyhow, Result};
use decklink::{
    find_decklink_with_capture_support, AudioSampleType, DisplayModeType, PixelFormat,
    SupportedVideoModeFlags, VideoConnection, VideoInputConversionMode, VideoInputFlags,
};

fn main() {
    test().unwrap()
}

pub fn test() -> Result<()> {
    let Some(decklink) = find_decklink_with_capture_support()? else {
        return Err(anyhow!("No decklink with capture support was detected."));
    };
    println!("{:#?}", decklink.info()?);
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
    input.start_streams()?;

    return Ok(());
}
