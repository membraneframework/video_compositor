#pragma once

#include "decklink/decklink_sdk/include/DeckLinkAPI.h"
#include "enums.h"

#include "decklink/src/api.rs.h"

rust::Vec<IDeckLinkPtr> get_decklinks();

VideoIOSupport into_video_io_support(int64_t value) noexcept;

IDeckLinkProfileAttributes *decklink_profile_attributes(IDeckLink *decklink);
IDeckLinkInput *decklink_input(IDeckLink *decklink);
void decklink_release(IDeckLink *decklink) noexcept;

bool profile_attributes_flag(IDeckLinkProfileAttributes *attrs,
                             FlagAttributeId id);
int64_t profile_attributes_integer(IDeckLinkProfileAttributes *attrs,
                                   IntegerAttributeId id);
double profile_attributes_float(IDeckLinkProfileAttributes *attrs,
                                FloatAttributeId id);
rust::String profile_attributes_string(IDeckLinkProfileAttributes *attrs,
                                       StringAttributeId id);
void profile_attributes_release(IDeckLinkProfileAttributes *attrs) noexcept;

DisplayMode
input_supports_video_mode(IDeckLinkInput *input, VideoConnection conn,
                          DisplayMode mode, PixelFormat pixel_format,
                          VideoInputConversionMode conversion_mode,
                          SupportedVideoModeFlags supported_mode_flags);
void input_enable_video_output(IDeckLinkInput *input, DisplayMode mode,
                               PixelFormat format, VideoInputFlags flags);
void input_enable_audio_output(IDeckLinkInput *input, uint32_t sample_rate,
                               AudioSampleType sample_type, uint32_t channels);
void input_set_callback(IDeckLinkInput *input, rust::Box<DynInputCallback> cb);
void input_start_streams(IDeckLinkInput *input);
void input_stop_streams(IDeckLinkInput *input);
void input_release(IDeckLinkInput *input) noexcept;

long video_input_frame_width(IDeckLinkVideoInputFrame* input) noexcept;
long video_input_frame_height(IDeckLinkVideoInputFrame* input) noexcept;
long video_input_frame_row_bytes(IDeckLinkVideoInputFrame* input) noexcept;
uint8_t* video_input_frame_bytes(IDeckLinkVideoInputFrame* input);
