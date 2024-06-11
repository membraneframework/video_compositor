use crate::{
    input_callback::{InputCallback, InputCallbackResult},
    DeckLinkError,
};

#[cxx::bridge]
mod ffi {
    #[derive(Debug)]
    struct IDeckLinkPtr {
        ptr: *mut IDeckLink,
    }

    #[derive(Debug)]
    struct VideoIOSupport {
        pub capture: bool,
        pub playback: bool,
    }

    #[repr(i64)]
    pub enum HResult {
        /// S_OK
        Ok = 0x00000000,
        /// S_FALSE
        False = 0x00000001,
        /// E_UNEXPECTED
        UnexpectedError = 0x8000FFFF,
        /// E_NOTIMPL
        NotImplementedError = 0x80000001,
        /// E_OUTOFMEMORY
        OutOfMemoryError = 0x80000002,
        /// E_INVALIDARG
        InvalidArg = 0x80000003,
        /// E_NOINTERFACE
        NoInterfaceError = 0x80000004,
        /// E_POINTER - required out parameter is set to nullptr
        NullPointerError = 0x80000005,
        /// E_HANDLE
        HandleError = 0x80000006,
        /// E_ABORT
        Abort = 0x80000007,
        /// E_FAIL
        Fail = 0x80000008,
        /// E_ACCESSDENIED
        AccessDenied = 0x80000009,
    }

    extern "Rust" {
        pub type DynInputCallback;
        unsafe fn video_input_frame_arrived(
            self: &DynInputCallback,
            video_frame: *mut IDeckLinkVideoInputFrame,
            audio_packet: *mut IDeckLinkAudioInputPacket,
        ) -> HResult;
    }

    unsafe extern "C++" {
        include!("decklink/cpp/api.h");

        type FlagAttributeId = crate::enums::ffi::FlagAttributeId;
        type IntegerAttributeId = crate::enums::ffi::IntegerAttributeId;
        type FloatAttributeId = crate::enums::ffi::FloatAttributeId;
        type StringAttributeId = crate::enums::ffi::StringAttributeId;
        type VideoConnection = crate::enums::ffi::VideoConnection;
        type DisplayMode = crate::enums::ffi::DisplayMode;
        type PixelFormat = crate::enums::ffi::PixelFormat;
        type VideoInputConversionMode = crate::enums::ffi::VideoInputConversionMode;

        type SupportedVideoModeFlags = crate::enums::ffi::SupportedVideoModeFlags;
        type VideoInputFlags = crate::enums::ffi::VideoInputFlags;

        type AudioSampleType = crate::enums::ffi::AudioSampleType;

        type IDeckLink;
        type IDeckLinkInput;
        type IDeckLinkProfileAttributes;
        type IDeckLinkVideoInputFrame;
        type IDeckLinkAudioInputPacket;
        type REFIID;

        fn get_decklinks() -> Result<Vec<IDeckLinkPtr>>;

        fn into_video_io_support(value: i64) -> VideoIOSupport;
    }

    // IDeckLink
    extern "C++" {
        unsafe fn decklink_profile_attributes(
            decklink: *mut IDeckLink,
        ) -> Result<*mut IDeckLinkProfileAttributes>;
        unsafe fn decklink_input(decklink: *mut IDeckLink) -> Result<*mut IDeckLinkInput>;
        unsafe fn decklink_release(decklink: *mut IDeckLink);
    }

    // IDeckLinkProfileAttributes
    extern "C++" {
        unsafe fn profile_attributes_flag(
            attrs: *mut IDeckLinkProfileAttributes,
            id: FlagAttributeId,
        ) -> Result<bool>;
        unsafe fn profile_attributes_integer(
            attrs: *mut IDeckLinkProfileAttributes,
            id: IntegerAttributeId,
        ) -> Result<i64>;
        unsafe fn profile_attributes_float(
            attrs: *mut IDeckLinkProfileAttributes,
            id: FloatAttributeId,
        ) -> Result<f64>;
        unsafe fn profile_attributes_string(
            attrs: *mut IDeckLinkProfileAttributes,
            id: StringAttributeId,
        ) -> Result<String>;
        unsafe fn profile_attributes_release(attrs: *mut IDeckLinkProfileAttributes);
    }

    // InputRef
    extern "C++" {
        unsafe fn input_supports_video_mode(
            input: *mut IDeckLinkInput,
            conn: VideoConnection,
            mode: DisplayMode,
            pixel_format: PixelFormat,
            conversion_mode: VideoInputConversionMode,
            supported_mode_flags: SupportedVideoModeFlags,
        ) -> Result<DisplayMode>;
        unsafe fn input_enable_video_output(
            input: *mut IDeckLinkInput,
            mode: DisplayMode,
            format: PixelFormat,
            flags: VideoInputFlags,
        ) -> Result<()>;
        unsafe fn input_enable_audio_output(
            input: *mut IDeckLinkInput,
            sample_rate: u32,
            sample_type: AudioSampleType,
            channels: u32,
        ) -> Result<()>;
        unsafe fn input_start_streams(input: *mut IDeckLinkInput) -> Result<()>;
        unsafe fn input_stop_streams(input: *mut IDeckLinkInput) -> Result<()>;
        unsafe fn input_set_callback(
            input: *mut IDeckLinkInput,
            cb: Box<DynInputCallback>,
        ) -> Result<()>;

        unsafe fn input_release(input: *mut IDeckLinkInput);
    }

    // IDeckLinkVideoInputFrame
    extern "C++" {
        unsafe fn video_input_frame_height(input: *mut IDeckLinkVideoInputFrame) -> i64;
        unsafe fn video_input_frame_width(input: *mut IDeckLinkVideoInputFrame) -> i64;
        unsafe fn video_input_frame_row_bytes(input: *mut IDeckLinkVideoInputFrame) -> i64;
        unsafe fn video_input_frame_bytes(input: *mut IDeckLinkVideoInputFrame) -> Result<*mut u8>;
    }
}

pub use ffi::{into_video_io_support, IntegerAttributeId};

pub struct DeckLink(*mut ffi::IDeckLink);

impl DeckLink {
    pub fn profile_attributes(&self) -> Result<ProfileAttributes, DeckLinkError> {
        let attrs = unsafe { ffi::decklink_profile_attributes(self.0) }?;
        Ok(ProfileAttributes(attrs))
    }
    pub fn input(&self) -> Result<Input, DeckLinkError> {
        let input = unsafe { ffi::decklink_input(self.0) }?;
        Ok(Input(input))
    }
}

impl Drop for DeckLink {
    fn drop(&mut self) {
        unsafe { ffi::decklink_release(self.0) }
    }
}

pub fn get_decklinks() -> Result<Vec<DeckLink>, DeckLinkError> {
    let ptrs = ffi::get_decklinks()?;
    Ok(ptrs
        .into_iter()
        .map(|wrapper| DeckLink(wrapper.ptr))
        .collect())
}

pub struct ProfileAttributes(*mut ffi::IDeckLinkProfileAttributes);

impl ProfileAttributes {
    pub fn get_flag(&self, id: ffi::FlagAttributeId) -> Result<bool, DeckLinkError> {
        unsafe { Ok(ffi::profile_attributes_flag(self.0, id)?) }
    }
    pub fn get_integer(&self, id: ffi::IntegerAttributeId) -> Result<i64, DeckLinkError> {
        unsafe { Ok(ffi::profile_attributes_integer(self.0, id)?) }
    }
    pub fn get_float(&self, id: ffi::FloatAttributeId) -> Result<f64, DeckLinkError> {
        unsafe { Ok(ffi::profile_attributes_float(self.0, id)?) }
    }
    pub fn get_string(&self, id: ffi::StringAttributeId) -> Result<String, DeckLinkError> {
        unsafe { Ok(ffi::profile_attributes_string(self.0, id)?) }
    }
}

impl Drop for ProfileAttributes {
    fn drop(&mut self) {
        unsafe { ffi::profile_attributes_release(self.0) }
    }
}

pub struct Input(*mut ffi::IDeckLinkInput);

impl Input {
    pub fn supports_video_mode(
        &self,
        conn: ffi::VideoConnection,
        mode: ffi::DisplayMode,
        pixel_format: ffi::PixelFormat,
        conversion_mode: ffi::VideoInputConversionMode,
        supported_mode_flags: ffi::SupportedVideoModeFlags,
    ) -> Result<ffi::DisplayMode, DeckLinkError> {
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
    pub fn enable_video_output(
        &mut self,
        mode: ffi::DisplayMode,
        format: ffi::PixelFormat,
        flags: ffi::VideoInputFlags,
    ) -> Result<(), DeckLinkError> {
        unsafe { Ok(ffi::input_enable_video_output(self.0, mode, format, flags)?) }
    }
    pub fn enable_audio_output(
        &mut self,
        sample_rate: u32,
        sample_type: ffi::AudioSampleType,
        channels: u32,
    ) -> Result<(), DeckLinkError> {
        unsafe {
            Ok(ffi::input_enable_audio_output(
                self.0,
                sample_rate,
                sample_type,
                channels,
            )?)
        }
    }
    pub fn start_streams(&mut self) -> Result<(), DeckLinkError> {
        unsafe { Ok(ffi::input_start_streams(self.0)?) }
    }
    pub fn stop_streams(&mut self) -> Result<(), DeckLinkError> {
        unsafe { Ok(ffi::input_stop_streams(self.0)?) }
    }
    pub fn set_callback(&mut self, cb: Box<dyn InputCallback>) -> Result<(), DeckLinkError> {
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
}

pub struct AudioInputPacket(*mut ffi::IDeckLinkAudioInputPacket);

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
}
