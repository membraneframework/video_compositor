#pragma once

#include "decklink/decklink_sdk/include/DeckLinkAPI.h"
#include "enums.h"

#include "decklink/src/api.rs.h"

rust::Vec<IDeckLinkPtr> get_decklinks();

VideoIOSupport into_video_io_support(int64_t value) noexcept;

IDeckLinkProfileAttributes *decklink_profile_attributes(IDeckLink *decklink);
IDeckLinkInput *decklink_input(IDeckLink *decklink);
IDeckLinkProfileManager *decklink_profile_manager(IDeckLink *decklink);
IDeckLinkConfiguration *decklink_configuration(IDeckLink *decklink);
void decklink_release(IDeckLink *decklink) noexcept;

// IDeckLinkProfileAttributes
bool profile_attributes_flag(IDeckLinkProfileAttributes *attrs,
                             FlagAttributeId id);
int64_t profile_attributes_integer(IDeckLinkProfileAttributes *attrs,
                                   IntegerAttributeId id);
double profile_attributes_float(IDeckLinkProfileAttributes *attrs,
                                FloatAttributeId id);
rust::String profile_attributes_string(IDeckLinkProfileAttributes *attrs,
                                       StringAttributeId id);
void profile_attributes_release(IDeckLinkProfileAttributes *attrs) noexcept;

// IDeckLinkInput
DisplayModeType
input_supports_video_mode(IDeckLinkInput *input, VideoConnection conn,
                          DisplayModeType mode, PixelFormat pixel_format,
                          VideoInputConversionMode conversion_mode,
                          SupportedVideoModeFlags supported_mode_flags);
void input_enable_video(IDeckLinkInput *input, DisplayModeType mode,
                        PixelFormat format, VideoInputFlags flags);
void input_enable_audio(IDeckLinkInput *input, uint32_t sample_rate,
                        AudioSampleType sample_type, uint32_t channels);
void input_set_callback(IDeckLinkInput *input, rust::Box<DynInputCallback> cb);
void input_start_streams(IDeckLinkInput *input);
void input_stop_streams(IDeckLinkInput *input);
void input_release(IDeckLinkInput *input) noexcept;

// IDeckLinkProfileManager
rust::Vec<IDeckLinkProfilePtr>
profile_manager_profiles(IDeckLinkProfileManager *manager);
void profile_manager_release(IDeckLinkProfileManager *manager) noexcept;

// IDeckLinkProfile
IDeckLinkProfileAttributes *
profile_profile_attributes(IDeckLinkProfile *profile);
bool profile_is_active(IDeckLinkProfile *profile);
void profile_release(IDeckLinkProfile *profile) noexcept;

// IDeckLinkConfiguration
bool configuration_flag(IDeckLinkConfiguration *conf, FlagConfigurationId id);
int64_t configuration_integer(IDeckLinkConfiguration *conf,
                              IntegerConfigurationId id);
double configuration_float(IDeckLinkConfiguration *conf,
                           FloatConfigurationId id);
rust::String configuration_string(IDeckLinkConfiguration *conf,
                                  StringConfigurationId id);
void configuration_set_flag(IDeckLinkConfiguration *conf,
                            FlagConfigurationId id, bool value);
void configuration_set_integer(IDeckLinkConfiguration *conf,
                               IntegerConfigurationId id, int64_t value);
void configuration_set_float(IDeckLinkConfiguration *conf,
                             FloatConfigurationId id, double value);
void configuration_set_string(IDeckLinkConfiguration *conf,
                              StringConfigurationId id, rust::String value);
void configuration_release(IDeckLinkConfiguration *conf) noexcept;

// IDeckLinkVideoInputFrame
long video_input_frame_width(IDeckLinkVideoInputFrame *input) noexcept;
long video_input_frame_height(IDeckLinkVideoInputFrame *input) noexcept;
long video_input_frame_row_bytes(IDeckLinkVideoInputFrame *input) noexcept;
uint8_t *video_input_frame_bytes(IDeckLinkVideoInputFrame *input);
BMDTimeValue video_input_frame_stream_time(IDeckLinkVideoInputFrame *input,
                                           BMDTimeScale time_scale);

// IDeckLinkAudioInputPacket
uint8_t *audio_input_packet_bytes(IDeckLinkAudioInputPacket *input);
int64_t
audio_input_packet_sample_count(IDeckLinkAudioInputPacket *input) noexcept;
BMDTimeValue audio_input_packet_packet_time(IDeckLinkAudioInputPacket *input,
                                            BMDTimeScale time_scale);

// IDeckLinkDisplayMode
int64_t display_mode_width(IDeckLinkDisplayMode* mode) noexcept;
int64_t display_mode_height(IDeckLinkDisplayMode* mode) noexcept;
rust::String display_mode_name(IDeckLinkDisplayMode* mode);
DisplayModeType display_mode_display_mode_type(IDeckLinkDisplayMode* mode);
Ratio display_mode_frame_rate(IDeckLinkDisplayMode* mode);
void display_mode_release(IDeckLinkDisplayMode* mode) noexcept;

