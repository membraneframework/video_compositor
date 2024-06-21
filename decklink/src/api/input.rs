use std::{mem::size_of, time::Duration};

use crate::{DeckLinkError, InputCallback, InputCallbackResult};

use super::{
    ffi::{self, PixelFormat},
    DisplayMode,
};

pub struct Input(pub(super) *mut ffi::IDeckLinkInput);

impl Input {
    pub fn supports_video_mode(
        &self,
        conn: ffi::VideoConnection,
        mode: ffi::DisplayModeType,
        pixel_format: ffi::PixelFormat,
        conversion_mode: ffi::VideoInputConversionMode,
        supported_mode_flags: ffi::SupportedVideoModeFlags,
    ) -> Result<ffi::DisplayModeType, DeckLinkError> {
        unsafe {
            Ok(ffi::input_supports_video_mode(
                self.0,
                conn,
                mode,
                pixel_format,
                conversion_mode,
                supported_mode_flags,
            )?)
        }
    }
    pub fn enable_video(
        &self,
        mode: ffi::DisplayModeType,
        format: ffi::PixelFormat,
        flags: ffi::VideoInputFlags,
    ) -> Result<(), DeckLinkError> {
        unsafe { Ok(ffi::input_enable_video(self.0, mode, format, flags)?) }
    }
    pub fn enable_audio(
        &self,
        sample_rate: u32,
        sample_type: ffi::AudioSampleType,
        channels: u32,
    ) -> Result<(), DeckLinkError> {
        unsafe {
            Ok(ffi::input_enable_audio(
                self.0,
                sample_rate,
                sample_type,
                channels,
            )?)
        }
    }
    pub fn start_streams(&self) -> Result<(), DeckLinkError> {
        unsafe { Ok(ffi::input_start_streams(self.0)?) }
    }
    pub fn stop_streams(&self) -> Result<(), DeckLinkError> {
        unsafe { Ok(ffi::input_stop_streams(self.0)?) }
    }
    pub fn pause_streams(&self) -> Result<(), DeckLinkError> {
        unsafe { Ok(ffi::input_pause_streams(self.0)?) }
    }
    pub fn flush_streams(&self) -> Result<(), DeckLinkError> {
        unsafe { Ok(ffi::input_flush_streams(self.0)?) }
    }
    pub fn set_callback(&self, cb: Box<dyn InputCallback>) -> Result<(), DeckLinkError> {
        let cb = Box::new(DynInputCallback::new(cb));
        unsafe { Ok(ffi::input_set_callback(self.0, cb)?) }
    }
}

impl Drop for Input {
    fn drop(&mut self) {
        unsafe { ffi::input_release(self.0) }
    }
}

unsafe impl Send for Input {}
unsafe impl Sync for Input {}

pub struct VideoInputFrame(*mut ffi::IDeckLinkVideoInputFrame);

impl VideoInputFrame {
    pub fn bytes(&self) -> Result<bytes::Bytes, DeckLinkError> {
        let height = unsafe { ffi::video_input_frame_height(self.0) as usize };
        let bytes_per_row = unsafe { ffi::video_input_frame_row_bytes(self.0) as usize };
        let mut data = bytes::BytesMut::zeroed(height * bytes_per_row);
        unsafe {
            let frame_ptr = ffi::video_input_frame_bytes(self.0)?;
            std::ptr::copy(frame_ptr, data.as_mut_ptr(), height * bytes_per_row);
        }
        Ok(data.freeze())
    }
    pub fn width(&self) -> usize {
        unsafe { ffi::video_input_frame_width(self.0) as usize }
    }
    pub fn height(&self) -> usize {
        unsafe { ffi::video_input_frame_height(self.0) as usize }
    }
    pub fn bytes_per_row(&self) -> usize {
        unsafe { ffi::video_input_frame_row_bytes(self.0) as usize }
    }
    pub fn pixel_format(&self) -> Result<PixelFormat, DeckLinkError> {
        Ok(unsafe { ffi::video_input_frame_pixel_format(self.0)? })
    }
    pub fn stream_time(&self) -> Result<Duration, DeckLinkError> {
        let time_value = unsafe { ffi::video_input_frame_stream_time(self.0, 1_000_000_000)? };
        Ok(Duration::from_nanos(time_value as u64))
    }
}

pub struct AudioInputPacket(*mut ffi::IDeckLinkAudioInputPacket);

impl AudioInputPacket {
    pub fn raw_bytes(
        &self,
        channels: usize,
        sample_type: ffi::AudioSampleType,
    ) -> Result<bytes::Bytes, DeckLinkError> {
        let sample_count = unsafe { ffi::audio_input_packet_sample_count(self.0) as usize };
        let bytes_per_sample = sample_type.repr / 8;
        let bytes_len = sample_count * channels * bytes_per_sample as usize;
        let mut data = bytes::BytesMut::zeroed(bytes_len);
        unsafe {
            let packet_ptr = ffi::audio_input_packet_bytes(self.0)?;
            std::ptr::copy(packet_ptr, data.as_mut_ptr(), bytes_len);
        }
        Ok(data.freeze())
    }

    pub fn as_32_bit_stereo(&self) -> Result<Vec<(i32, i32)>, DeckLinkError> {
        const _SIZE_ASSERT: () = assert!(size_of::<(i32, i32)>() == 8);

        let sample_count = unsafe { ffi::audio_input_packet_sample_count(self.0) as usize };
        let mut result: Vec<(i32, i32)> = Vec::with_capacity(sample_count);
        unsafe {
            let packet_ptr = ffi::audio_input_packet_bytes(self.0)?;
            let packet_ptr = packet_ptr as *mut (i32, i32);

            for index in 0..sample_count {
                result.push(*packet_ptr.offset(index as isize))
            }
        }
        Ok(result)
    }

    pub fn as_16_bit_stereo(&self) -> Result<Vec<(i16, i16)>, DeckLinkError> {
        const _SIZE_ASSERT: () = assert!(size_of::<(i16, i16)>() == 4);

        let sample_count = unsafe { ffi::audio_input_packet_sample_count(self.0) as usize };
        let mut result: Vec<(i16, i16)> = Vec::with_capacity(sample_count);
        unsafe {
            let packet_ptr = ffi::audio_input_packet_bytes(self.0)?;
            let packet_ptr = packet_ptr as *mut (i16, i16);

            for index in 0..sample_count {
                result.push(*packet_ptr.offset(index as isize))
            }
        }
        Ok(result)
    }

    pub fn packet_time(&self) -> Result<Duration, DeckLinkError> {
        let time_value = unsafe { ffi::audio_input_packet_packet_time(self.0, 1_000_000_000)? };
        Ok(Duration::from_nanos(time_value as u64))
    }
}

pub(crate) struct DynInputCallback(Box<dyn InputCallback + 'static>);

impl DynInputCallback {
    fn new(cb: Box<dyn InputCallback + 'static>) -> DynInputCallback {
        DynInputCallback(cb)
    }
    pub(crate) unsafe fn video_input_frame_arrived(
        self: &DynInputCallback,
        video_frame: *mut ffi::IDeckLinkVideoInputFrame,
        audio_packet: *mut ffi::IDeckLinkAudioInputPacket,
    ) -> ffi::HResult {
        let mut video = None;
        if !video_frame.is_null() {
            video = Some(VideoInputFrame(video_frame))
        }

        let mut audio = None;
        if !audio_packet.is_null() {
            audio = Some(AudioInputPacket(audio_packet))
        }
        let result = self
            .0
            .video_input_frame_arrived(video.as_mut(), audio.as_mut());
        match result {
            InputCallbackResult::Ok => ffi::HResult::Ok,
            InputCallbackResult::Failure => ffi::HResult::Fail,
        }
    }

    pub(crate) fn video_input_format_changed(
        self: &DynInputCallback,
        events: ffi::VideoInputFormatChangedEvents,
        display_mode: *mut ffi::IDeckLinkDisplayMode,
        flags: ffi::DetectedVideoInputFormatFlags,
    ) -> ffi::HResult {
        let result =
            self.0
                .video_input_format_changed(events, DisplayMode(display_mode, false), flags);

        match result {
            InputCallbackResult::Ok => ffi::HResult::Ok,
            InputCallbackResult::Failure => ffi::HResult::Fail,
        }
    }
}
