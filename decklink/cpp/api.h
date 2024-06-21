#pragma once

#include "decklink/decklink_sdk/include/DeckLinkAPI.h"
#include "enums.h"

#include "decklink/src/api.rs.h"
#include <cstdint>

rust::Vec<IDeckLinkPtr> get_decklinks();

VideoIOSupport into_video_io_support(int64_t value);

IDeckLinkProfileAttributes *decklink_profile_attributes(IDeckLink *decklink);
IDeckLinkInput *decklink_input(IDeckLink *decklink);
IDeckLinkProfileManager *decklink_profile_manager(IDeckLink *decklink);
IDeckLinkConfiguration *decklink_configuration(IDeckLink *decklink);
void decklink_release(IDeckLink *decklink);

// IDeckLinkProfileAttributes
HResult profile_attributes_flag(IDeckLinkProfileAttributes *attrs,
                                FlagAttributeId id, bool &out);
HResult profile_attributes_integer(IDeckLinkProfileAttributes *attrs,
                                   IntegerAttributeId id, int64_t &out);
HResult profile_attributes_float(IDeckLinkProfileAttributes *attrs,
                                 FloatAttributeId id, double &out);
HResult profile_attributes_string(IDeckLinkProfileAttributes *attrs,
                                  StringAttributeId id, rust::String &out,
                                  bool is_static);
HResult profile_attributes_release(IDeckLinkProfileAttributes *attrs);

// IDeckLinkInput
DisplayModeType
input_supports_video_mode(IDeckLinkInput *input, VideoConnection conn,
                          DisplayModeType mode, PixelFormat pixel_format,
                          VideoInputConversionMode conversion_mode,
                          SupportedVideoModeFlags supported_mode_flags);
HResult input_enable_video(IDeckLinkInput *input, DisplayModeType mode,
                        PixelFormat format, VideoInputFlags flags);
HResult input_enable_audio(IDeckLinkInput *input, uint32_t sample_rate,
                        AudioSampleType sample_type, uint32_t channels);
HResult input_set_callback(IDeckLinkInput *input, rust::Box<DynInputCallback> cb);
HResult input_start_streams(IDeckLinkInput *input);
HResult input_stop_streams(IDeckLinkInput *input);
HResult input_pause_streams(IDeckLinkInput *input);
HResult input_flush_streams(IDeckLinkInput *input);
HResult input_release(IDeckLinkInput *input);

// IDeckLinkProfileManager
rust::Vec<IDeckLinkProfilePtr>
profile_manager_profiles(IDeckLinkProfileManager *manager);
void profile_manager_release(IDeckLinkProfileManager *manager);

// IDeckLinkProfile
IDeckLinkProfileAttributes *
profile_profile_attributes(IDeckLinkProfile *profile);
bool profile_is_active(IDeckLinkProfile *profile);
void profile_release(IDeckLinkProfile *profile);

// IDeckLinkConfiguration
HResult configuration_flag(IDeckLinkConfiguration *conf, FlagConfigurationId id,
                           bool &out);
HResult configuration_integer(IDeckLinkConfiguration *conf,
                              IntegerConfigurationId id, int64_t &out);
HResult configuration_float(IDeckLinkConfiguration *conf,
                            FloatConfigurationId id, double &out);
HResult configuration_string(IDeckLinkConfiguration *conf,
                             StringConfigurationId id, rust::String &out);
HResult configuration_set_flag(IDeckLinkConfiguration *conf,
                               FlagConfigurationId id, bool value);
HResult configuration_set_integer(IDeckLinkConfiguration *conf,
                                  IntegerConfigurationId id, int64_t value);
HResult configuration_set_float(IDeckLinkConfiguration *conf,
                                FloatConfigurationId id, double value);
HResult configuration_set_string(IDeckLinkConfiguration *conf,
                                 StringConfigurationId id, rust::String value);
HResult configuration_release(IDeckLinkConfiguration *conf);

// IDeckLinkVideoInputFrame
long video_input_frame_width(IDeckLinkVideoInputFrame *frame);
long video_input_frame_height(IDeckLinkVideoInputFrame *frame);
long video_input_frame_row_bytes(IDeckLinkVideoInputFrame *frame);
uint8_t *video_input_frame_bytes(IDeckLinkVideoInputFrame *frame);
PixelFormat video_input_frame_pixel_format(IDeckLinkVideoInputFrame *frame);
BMDTimeValue video_input_frame_stream_time(IDeckLinkVideoInputFrame *frame,
                                           BMDTimeScale time_scale);

// IDeckLinkAudioInputPacket
uint8_t *audio_input_packet_bytes(IDeckLinkAudioInputPacket *input);
int64_t audio_input_packet_sample_count(IDeckLinkAudioInputPacket *input);
BMDTimeValue audio_input_packet_packet_time(IDeckLinkAudioInputPacket *input,
                                            BMDTimeScale time_scale);

// IDeckLinkDisplayMode
int64_t display_mode_width(IDeckLinkDisplayMode *mode);
int64_t display_mode_height(IDeckLinkDisplayMode *mode);
rust::String display_mode_name(IDeckLinkDisplayMode *mode);
DisplayModeType display_mode_display_mode_type(IDeckLinkDisplayMode *mode);
Ratio display_mode_frame_rate(IDeckLinkDisplayMode *mode);
void display_mode_release(IDeckLinkDisplayMode *mode);
