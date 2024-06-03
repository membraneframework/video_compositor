#pragma once

#include "decklink/decklink_sdk/include/DeckLinkAPI.h"
#include "decklink/src/enums.rs.h"

REFIID declare_id(DeclarationId id);

BMDDeckLinkAttributeID flag_attribute_id(FlagAttributeId id);
BMDDeckLinkAttributeID integer_attribute_id(IntegerAttributeId id);
BMDDeckLinkAttributeID float_attribute_id(FloatAttributeId id);
BMDDeckLinkAttributeID string_attribute_id(StringAttributeId id);

// Naming convention for function below:
// - "from_" refers to function that converts shared (Rust-C++) type to BlackMagic SDK type.
// - "into_" refers to function that convers BlackMagic SDK type into shared (Rust-C++) type.

BMDVideoConnection from_video_connection(VideoConnection);
VideoConnection into_video_connection(BMDVideoConnection);

BMDAudioConnection from_audio_connection(AudioConnection);
AudioConnection into_audio_connection(BMDAudioConnection);

BMDDisplayMode from_display_mode(DisplayMode);
DisplayMode into_display_mode(BMDDisplayMode);

BMDPixelFormat from_pixel_format(PixelFormat);
PixelFormat into_pixel_format(BMDPixelFormat);

BMDVideoInputConversionMode from_video_input_conversion_mode(VideoInputConversionMode);
VideoInputConversionMode into_video_input_conversion_mode(BMDVideoInputConversionMode);

BMDSupportedVideoModeFlags from_supported_video_mode_flags(SupportedVideoModeFlags);
SupportedVideoModeFlags into_supported_video_mode_flags(BMDSupportedVideoModeFlags);

BMDVideoInputFlags from_video_input_flags(VideoInputFlags);
VideoInputFlags into_video_input_flags(BMDVideoInputFlags);
