use std::ptr::null_mut;

use crate::DeckLinkError;

use input::DynInputCallback;

pub(super) mod device;
pub(super) mod input;
pub(super) mod profile;

#[cxx::bridge]
mod ffi {
    #[derive(Debug)]
    struct IDeckLinkPtr {
        ptr: *mut IDeckLink,
    }
    #[derive(Debug)]
    struct IDeckLinkProfilePtr {
        ptr: *mut IDeckLinkProfile,
    }

    #[derive(Debug)]
    struct VideoIOSupport {
        pub capture: bool,
        pub playback: bool,
    }

    #[derive(Debug)]
    struct Ratio {
        pub num: i64,
        pub den: i64,
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
        unsafe fn video_input_format_changed(
            self: &DynInputCallback,
            events: VideoInputFormatChangedEvents,
            display_mode: *mut IDeckLinkDisplayMode,
            flags: DetectedVideoInputFormatFlags,
        ) -> HResult;
    }

    unsafe extern "C++" {
        include!("decklink/cpp/api.h");

        type FlagAttributeId = crate::enums::ffi::FlagAttributeId;
        type IntegerAttributeId = crate::enums::ffi::IntegerAttributeId;
        type FloatAttributeId = crate::enums::ffi::FloatAttributeId;
        type StringAttributeId = crate::enums::ffi::StringAttributeId;

        type FlagConfigurationId = crate::enums::ffi::FlagConfigurationId;
        type IntegerConfigurationId = crate::enums::ffi::IntegerConfigurationId;
        type FloatConfigurationId = crate::enums::ffi::FloatConfigurationId;
        type StringConfigurationId = crate::enums::ffi::StringConfigurationId;

        type VideoConnection = crate::enums::ffi::VideoConnection;
        type DisplayModeType = crate::enums::ffi::DisplayModeType;
        type PixelFormat = crate::enums::ffi::PixelFormat;
        type VideoInputConversionMode = crate::enums::ffi::VideoInputConversionMode;
        type VideoInputFormatChangedEvents = crate::enums::ffi::VideoInputFormatChangedEvents;
        type DetectedVideoInputFormatFlags = crate::enums::ffi::DetectedVideoInputFormatFlags;

        type SupportedVideoModeFlags = crate::enums::ffi::SupportedVideoModeFlags;
        type VideoInputFlags = crate::enums::ffi::VideoInputFlags;

        type AudioSampleType = crate::enums::ffi::AudioSampleType;

        type IDeckLink;
        type IDeckLinkInput;
        type IDeckLinkProfile;
        type IDeckLinkProfileManager;
        type IDeckLinkProfileAttributes;
        type IDeckLinkConfiguration;
        type IDeckLinkVideoInputFrame;
        type IDeckLinkAudioInputPacket;
        type IDeckLinkDisplayMode;
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
        unsafe fn decklink_profile_manager(
            decklink: *mut IDeckLink,
        ) -> Result<*mut IDeckLinkProfileManager>;
        unsafe fn decklink_configuration(
            decklink: *mut IDeckLink,
        ) -> Result<*mut IDeckLinkConfiguration>;
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
            mode: DisplayModeType,
            pixel_format: PixelFormat,
            conversion_mode: VideoInputConversionMode,
            supported_mode_flags: SupportedVideoModeFlags,
        ) -> Result<DisplayModeType>;
        unsafe fn input_enable_video(
            input: *mut IDeckLinkInput,
            mode: DisplayModeType,
            format: PixelFormat,
            flags: VideoInputFlags,
        ) -> Result<()>;
        unsafe fn input_enable_audio(
            input: *mut IDeckLinkInput,
            sample_rate: u32,
            sample_type: AudioSampleType,
            channels: u32,
        ) -> Result<()>;
        unsafe fn input_start_streams(input: *mut IDeckLinkInput) -> Result<()>;
        unsafe fn input_stop_streams(input: *mut IDeckLinkInput) -> Result<()>;
        unsafe fn input_pause_streams(input: *mut IDeckLinkInput) -> Result<()>;
        unsafe fn input_flush_streams(input: *mut IDeckLinkInput) -> Result<()>;
        unsafe fn input_set_callback(
            input: *mut IDeckLinkInput,
            cb: Box<DynInputCallback>,
        ) -> Result<()>;

        unsafe fn input_release(input: *mut IDeckLinkInput);
    }

    // IDeckLinkProfileManager
    extern "C++" {
        unsafe fn profile_manager_profiles(
            manger: *mut IDeckLinkProfileManager,
        ) -> Result<Vec<IDeckLinkProfilePtr>>;
        unsafe fn profile_manager_release(manager: *mut IDeckLinkProfileManager);
    }

    // IDeckLinkProfile
    extern "C++" {
        unsafe fn profile_profile_attributes(
            profile: *mut IDeckLinkProfile,
        ) -> Result<*mut IDeckLinkProfileAttributes>;
        unsafe fn profile_is_active(profile: *mut IDeckLinkProfile) -> Result<bool>;
        unsafe fn profile_release(profile: *mut IDeckLinkProfile);
    }

    // IDeckLinkConfiguration
    extern "C++" {
        unsafe fn configuration_flag(
            conf: *mut IDeckLinkConfiguration,
            id: FlagConfigurationId,
        ) -> Result<bool>;
        unsafe fn configuration_integer(
            conf: *mut IDeckLinkConfiguration,
            id: IntegerConfigurationId,
        ) -> Result<i64>;
        unsafe fn configuration_float(
            conf: *mut IDeckLinkConfiguration,
            id: FloatConfigurationId,
        ) -> Result<f64>;
        unsafe fn configuration_string(
            conf: *mut IDeckLinkConfiguration,
            id: StringConfigurationId,
        ) -> Result<String>;
        unsafe fn configuration_set_flag(
            conf: *mut IDeckLinkConfiguration,
            id: FlagConfigurationId,
            value: bool,
        ) -> Result<()>;
        unsafe fn configuration_set_integer(
            conf: *mut IDeckLinkConfiguration,
            id: IntegerConfigurationId,
            value: i64,
        ) -> Result<()>;
        unsafe fn configuration_set_float(
            conf: *mut IDeckLinkConfiguration,
            id: FloatConfigurationId,
            value: f64,
        ) -> Result<()>;
        unsafe fn configuration_set_string(
            conf: *mut IDeckLinkConfiguration,
            id: StringConfigurationId,
            value: String,
        ) -> Result<()>;
        unsafe fn configuration_release(conf: *mut IDeckLinkConfiguration);
    }

    // IDeckLinkVideoInputFrame
    extern "C++" {
        unsafe fn video_input_frame_height(input: *mut IDeckLinkVideoInputFrame) -> i64;
        unsafe fn video_input_frame_width(input: *mut IDeckLinkVideoInputFrame) -> i64;
        unsafe fn video_input_frame_row_bytes(input: *mut IDeckLinkVideoInputFrame) -> i64;
        unsafe fn video_input_frame_bytes(input: *mut IDeckLinkVideoInputFrame) -> Result<*mut u8>;
        unsafe fn video_input_frame_stream_time(
            input: *mut IDeckLinkVideoInputFrame,
            time_scale: i64,
        ) -> Result<i64>;
    }

    // IDeckLinkAudioInputPacket
    extern "C++" {
        unsafe fn audio_input_packet_bytes(
            input: *mut IDeckLinkAudioInputPacket,
        ) -> Result<*mut u8>;
        unsafe fn audio_input_packet_sample_count(input: *mut IDeckLinkAudioInputPacket) -> i64;
        unsafe fn audio_input_packet_packet_time(
            input: *mut IDeckLinkAudioInputPacket,
            time_scale: i64,
        ) -> Result<i64>;
    }

    // IDeckLinkDisplayMode
    extern "C++" {
        unsafe fn display_mode_width(mode: *mut IDeckLinkDisplayMode) -> i64;
        unsafe fn display_mode_height(mode: *mut IDeckLinkDisplayMode) -> i64;
        unsafe fn display_mode_name(mode: *mut IDeckLinkDisplayMode) -> Result<String>;
        unsafe fn display_mode_display_mode_type(
            mode: *mut IDeckLinkDisplayMode,
        ) -> Result<DisplayModeType>;
        unsafe fn display_mode_frame_rate(mode: *mut IDeckLinkDisplayMode) -> Result<Ratio>;

        unsafe fn display_mode_release(mode: *mut IDeckLinkDisplayMode);
    }
}

pub use ffi::{into_video_io_support, VideoIOSupport};

use self::{
    device::DeckLinkConfiguration,
    input::Input,
    profile::{ProfileAttributes, ProfileManager},
};

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

    pub fn profile_manager(&self) -> Result<Option<ProfileManager>, DeckLinkError> {
        let manager = unsafe { ffi::decklink_profile_manager(self.0) }?;
        if manager == null_mut() {
            Ok(None)
        } else {
            Ok(Some(ProfileManager(manager)))
        }
    }

    pub fn configuration(&self) -> Result<DeckLinkConfiguration, DeckLinkError> {
        let configuration = unsafe { ffi::decklink_configuration(self.0) }?;
        Ok(DeckLinkConfiguration(configuration))
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

pub struct DisplayMode(*mut ffi::IDeckLinkDisplayMode, bool);

impl DisplayMode {
    pub fn width(&self) -> usize {
        unsafe { ffi::display_mode_width(self.0) as usize }
    }

    pub fn height(&self) -> usize {
        unsafe { ffi::display_mode_height(self.0) as usize }
    }

    pub fn name(&self) -> Result<String, DeckLinkError> {
        Ok(unsafe { ffi::display_mode_name(self.0) }?)
    }

    pub fn display_mode_type(&self) -> Result<ffi::DisplayModeType, DeckLinkError> {
        Ok(unsafe { ffi::display_mode_display_mode_type(self.0) }?)
    }

    pub fn display_mode_frame_rate(&self) -> Result<ffi::Ratio, DeckLinkError> {
        Ok(unsafe { ffi::display_mode_frame_rate(self.0) }?)
    }
}

impl Drop for DisplayMode {
    fn drop(&mut self) {
        if self.1 {
            unsafe { ffi::display_mode_release(self.0) }
        }
    }
}
