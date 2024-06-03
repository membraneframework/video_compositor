// WARNING: When adding any value here, make sure to also update "cpp/enums.cpp".

#[cxx::bridge]
pub mod ffi {
    #[derive(Debug, Copy, Clone)]
    pub enum DeclarationId {
        VideoOutputCallback,
        InputCallback,
        EncoderInputCallback,
        MemoryAllocator,
        AudioOutputCallback,
        Iterator,
        APIInformation,
        Output,
        Input,
        HDMIInputEDID,
        EncoderInput,
        VideoFrame,
        MutableVideoFrame,
        VideoFrame3DExtensions,
        VideoFrameMetadataExtensions,
        VideoInputFrame,
        AncillaryPacket,
        AncillaryPacketIterator,
        VideoFrameAncillaryPackets,
        VideoFrameAncillary,
        EncoderPacket,
        EncoderVideoPacket,
        EncoderAudioPacket,
        H265NALPacket,
        AudioInputPacket,
        ScreenPreviewCallback,
        GLScreenPreviewHelper,
        NotificationCallback,
        Notification,
        ProfileAttributes,
        ProfileIterator,
        Profile,
        ProfileCallback,
        ProfileManager,
        Status,
        Keyer,
        VideoConversion,
        DeviceNotificationCallback,
        Discovery,
    }

    #[derive(Debug, Copy, Clone)]
    pub enum FlagAttributeId {
        SupportsInternalKeying,
        SupportsExternalKeying,
        SupportsInputFormatDetection,
        HasReferenceInput,
        HasSerialPort,
        HasAnalogVideoOutputGain,
        CanOnlyAdjustOverallVideoOutputGain,
        HasVideoInputAntiAliasingFilter,
        HasBypass,
        SupportsClockTimingAdjustment,
        SupportsFullFrameReferenceInputTimingOffset,
        SupportsSMPTELevelAOutput,
        SupportsAutoSwitchingPPsFOnInput,
        SupportsDualLinkSDI,
        SupportsQuadLinkSDI,
        SupportsIdleOutput,
        VANCRequires10BitYUVVideoFrames,
        HasLTCTimecodeInput,
        SupportsHDRMetadata,
        SupportsColorspaceMetadata,
        SupportsHDMITimecode,
        SupportsHighFrameRateTimecode,
        SupportsSynchronizeToCaptureGroup,
        SupportsSynchronizeToPlaybackGroup,
        HasMonitorOut,
    }

    #[derive(Debug, Copy, Clone)]
    pub enum IntegerAttributeId {
        MaximumAudioChannels,
        MaximumHDMIAudioChannels,
        MaximumAnalogAudioInputChannels,
        MaximumAnalogAudioOutputChannels,
        NumberOfSubDevices,
        SubDeviceIndex,
        PersistentID,
        DeviceGroupID,
        TopologicalID,
        VideoOutputConnections, // Returns a BMDVideoConnection bit field
        VideoInputConnections,  // Returns a BMDVideoConnection bit field
        AudioOutputConnections, // Returns a BMDVideoConnection bit field
        AudioInputConnections,  // Returns a BMDVideoConnection bit field
        VideoIOSupport,         // Returns a BMDVideoIOSupport bit field
        DeckControlConnections, // Returns a BMDDeckControlConnection bit field
        DeviceInterface,        // Returns a BMDDeviceInterface
        AudioInputRCAChannelCount,
        AudioInputXLRChannelCount,
        AudioOutputRCAChannelCount,
        AudioOutputXLRChannelCount,
        ProfileID, // Returns a BMDProfileID
        Duplex,
        MinimumPrerollFrames,
        SupportedDynamicRange,
        MezzanineType,
    }

    #[derive(Debug, Copy, Clone)]
    pub enum FloatAttributeId {
        VideoInputGainMinimum,
        VideoInputGainMaximum,
        VideoOutputGainMinimum,
        VideoOutputGainMaximum,
        MicrophoneInputGainMinimum,
        MicrophoneInputGainMaximum,
    }

    #[derive(Debug, Copy, Clone)]
    pub enum StringAttributeId {
        SerialPortDeviceName,
        VendorName,
        DisplayName,
        ModelName,
        DeviceHandle,
        EthernetMACAddress,
    }

    #[derive(Debug, Copy, Clone)]
    pub enum VideoConnection {
        Unspecified,
        SDI,
        HDMI,
        OpticalSDI,
        Component,
        Composite,
        SVideo,
        Ethernet,
        OpticalEthernet,
    }

    #[derive(Debug, Copy, Clone)]
    pub enum AudioConnection {
        Embedded,
        AESEBU,
        Analog,
        AnalogXLR,
        AnalogRCA,
        Microphone,
        Headphones,
    }

    #[derive(Debug, Copy, Clone)]
    pub enum DisplayMode {
        /* SD Modes */
        ModeNTSC,
        ModeNTSC2398, // 3:2 pulldown
        ModePAL,
        ModeNTSCp,
        ModePALp,

        /* HD 1080 Modes */
        ModeHD1080p2398,
        ModeHD1080p24,
        ModeHD1080p25,
        ModeHD1080p2997,
        ModeHD1080p30,
        ModeHD1080p4795,
        ModeHD1080p48,
        ModeHD1080p50,
        ModeHD1080p5994,
        ModeHD1080p6000, // N.B. This _really_ is 60.00 Hz.
        ModeHD1080p9590,
        ModeHD1080p96,
        ModeHD1080p100,
        ModeHD1080p11988,
        ModeHD1080p120,
        ModeHD1080i50,
        ModeHD1080i5994,
        ModeHD1080i6000, // N.B. This _really_ is 60.00 Hz.

        /* HD 720 Modes */
        ModeHD720p50,
        ModeHD720p5994,
        ModeHD720p60,

        /* 2K Modes */
        Mode2k2398,
        Mode2k24,
        Mode2k25,

        /* 2K DCI Modes */
        Mode2kDCI2398,
        Mode2kDCI24,
        Mode2kDCI25,
        Mode2kDCI2997,
        Mode2kDCI30,
        Mode2kDCI4795,
        Mode2kDCI48,
        Mode2kDCI50,
        Mode2kDCI5994,
        Mode2kDCI60,
        Mode2kDCI9590,
        Mode2kDCI96,
        Mode2kDCI100,
        Mode2kDCI11988,
        Mode2kDCI120,

        /* 4K UHD Modes */
        Mode4K2160p2398,
        Mode4K2160p24,
        Mode4K2160p25,
        Mode4K2160p2997,
        Mode4K2160p30,
        Mode4K2160p4795,
        Mode4K2160p48,
        Mode4K2160p50,
        Mode4K2160p5994,
        Mode4K2160p60,
        Mode4K2160p9590,
        Mode4K2160p96,
        Mode4K2160p100,
        Mode4K2160p11988,
        Mode4K2160p120,

        /* 4K DCI Modes */
        Mode4kDCI2398,
        Mode4kDCI24,
        Mode4kDCI25,
        Mode4kDCI2997,
        Mode4kDCI30,
        Mode4kDCI4795,
        Mode4kDCI48,
        Mode4kDCI50,
        Mode4kDCI5994,
        Mode4kDCI60,
        Mode4kDCI9590,
        Mode4kDCI96,
        Mode4kDCI100,
        Mode4kDCI11988,
        Mode4kDCI120,

        /* 8K UHD Modes */
        Mode8K4320p2398,
        Mode8K4320p24,
        Mode8K4320p25,
        Mode8K4320p2997,
        Mode8K4320p30,
        Mode8K4320p4795,
        Mode8K4320p48,
        Mode8K4320p50,
        Mode8K4320p5994,
        Mode8K4320p60,

        /* 8K DCI Modes */
        Mode8kDCI2398,
        Mode8kDCI24,
        Mode8kDCI25,
        Mode8kDCI2997,
        Mode8kDCI30,
        Mode8kDCI4795,
        Mode8kDCI48,
        Mode8kDCI50,
        Mode8kDCI5994,
        Mode8kDCI60,

        /* PC Modes */
        Mode640x480p60,
        Mode800x600p60,
        Mode1440x900p50,
        Mode1440x900p60,
        Mode1440x1080p50,
        Mode1440x1080p60,
        Mode1600x1200p50,
        Mode1600x1200p60,
        Mode1920x1200p50,
        Mode1920x1200p60,
        Mode1920x1440p50,
        Mode1920x1440p60,
        Mode2560x1440p50,
        Mode2560x1440p60,
        Mode2560x1600p50,
        Mode2560x1600p60,

        /* Special Modes */
        ModeUnknown,
    }

    #[derive(Debug, Copy, Clone)]
    pub enum PixelFormat {
        FormatUnspecified,
        Format8BitYUV,
        Format10BitYUV,
        Format10BitYUVA, // Big-endian YUVA 10 bit per component with SMPTE video levels (64-940) for YUV but full range alpha
        Format8BitARGB,
        Format8BitBGRA,
        Format10BitRGB, // Big-endian RGB 10-bit per component with SMPTE video levels (64-940). Packed as 2:10:10:10
        Format12BitRGB, // Big-endian RGB 12-bit per component with full range (0-4095). Packed as 12-bit per component
        Format12BitRGBLE, // Little-endian RGB 12-bit per component with full range (0-4095). Packed as 12-bit per component
        Format10BitRGBXLE, // Little-endian 10-bit RGB with SMPTE video levels (64-940)
        Format10BitRGBX,  // Big-endian 10-bit RGB with SMPTE video levels (64-940)
        FormatH265,       // High Efficiency Video Coding (HEVC/h.265)

        /* AVID DNxHR */
        FormatDNxHR,
    }

    #[derive(Debug, Copy, Clone)]
    pub enum VideoInputConversionMode {
        NoVideoInputConversion,
        VideoInputLetterboxDownconversionFromHD1080,
        VideoInputAnamorphicDownconversionFromHD1080,
        VideoInputLetterboxDownconversionFromHD720,
        VideoInputAnamorphicDownconversionFromHD720,
        VideoInputLetterboxUpconversion,
        VideoInputAnamorphicUpconversion,
    }

    #[derive(Debug, Copy, Clone, Default)]
    pub struct SupportedVideoModeFlags {
        pub supports_keying: bool,
        pub supports_dual_stream_3d: bool,
        pub supports_SDI_single_link: bool,
        pub supports_SDI_dual_link: bool,
        pub supports_SDI_quad_link: bool,
        pub supports_in_any_profile: bool,
        pub supports_PsF: bool,
    }

    #[derive(Debug, Copy, Clone, Default)]
    pub struct VideoInputFlags {
        pub enable_format_detection: bool,
        pub dual_stream_3d: bool,
        pub synchronize_to_capture_group: bool,
    }

    #[repr(u32)]
    pub enum AudioSampleType {
        Sample16bit = 16,
        Sample32bit = 32,
    }
}
