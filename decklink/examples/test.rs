use anyhow::{anyhow, Result};
use decklink::{
    find_decklink_with_capture_support, AudioInputPacket, AudioSampleType, DisplayMode, InputCallback, InputCallbackResult, PixelFormat, SupportedVideoModeFlags, VideoConnection, VideoInputConversionMode, VideoInputFlags, VideoInputFrame
};

fn main() {
    test().unwrap()
}

struct ChannelInputWrapper {
    //
}

impl InputCallback for ChannelInputWrapper {
    fn video_input_frame_arrived(
        &self,
        video_frame: Option<&mut VideoInputFrame>,
        audio_packet: Option<&mut AudioInputPacket>,
    ) -> InputCallbackResult {
        InputCallbackResult::Ok
    }
}

pub fn test() -> Result<()> {
    let Some(decklink) = find_decklink_with_capture_support()? else {
        return Err(anyhow!("No decklink with capture support was detected."));
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
    input.set_callback(Box::new(ChannelInputWrapper {}))?;
    input.stop_streams()?;

    return Ok(());
}
