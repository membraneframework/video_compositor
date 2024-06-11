#include "api.h"
#include "callback.h"
#include "decklink/decklink_sdk/include/DeckLinkAPI.h"
#include "enums.h"

#include "decklink/src/api.rs.h"
#include "decklink/src/enums.rs.h"
#include <cstdint>
#include <stdexcept>

rust::Vec<IDeckLinkPtr> get_decklinks() {
  auto deckLinkIterator = CreateDeckLinkIteratorInstance();
  if (!deckLinkIterator) {
    throw "This application requires the DeckLink drivers installed.";
  }

  rust::Vec<IDeckLinkPtr> deckLinks;
  while (true) {
    IDeckLink *deckLink;
    HRESULT result = deckLinkIterator->Next(&deckLink);
    if (result != S_OK) {
      deckLinkIterator->Release();
      return deckLinks;
    }
    deckLinks.push_back(IDeckLinkPtr{deckLink});
  }
}

VideoIOSupport into_video_io_support(int64_t value) noexcept {
  VideoIOSupport state;
  if ((bmdDeviceSupportsCapture & value) != 0) {
    state.capture = true;
  }
  if ((bmdDeviceSupportsPlayback & value) != 0) {
    state.playback = true;
  }
  return state;
}

IDeckLinkProfileAttributes *decklink_profile_attributes(IDeckLink *decklink) {
  IDeckLinkProfileAttributes *attributes = nullptr;

  HRESULT result = decklink->QueryInterface(IID_IDeckLinkProfileAttributes,
                                            (void **)&attributes);
  if (result != S_OK) {
    throw std::runtime_error(
        "IDeckLink::QueryInterface IID_IDeckLinkProfileAttributes failed.");
  }
  return attributes;
}

IDeckLinkInput *decklink_input(IDeckLink *decklink) {
  IDeckLinkInput *input = nullptr;

  HRESULT result =
      decklink->QueryInterface(IID_IDeckLinkInput, (void **)&input);
  if (result != S_OK) {
    throw std::runtime_error(
        "IDeckLink::QueryInterface IID_IDeckLinkProfileAttributes failed.");
  }
  return input;
}

void decklink_release(IDeckLink *decklink) noexcept { decklink->Release(); }

//
// IDeckLinkProfileAttributes
//

bool profile_attributes_flag(IDeckLinkProfileAttributes *attrs,
                             FlagAttributeId id) {
  bool value;
  if (attrs->GetFlag(flag_attribute_id(id), &value) != S_OK) {
    throw std::runtime_error("IDeckLinkProfileAttributes::GetFlag failed.");
  }
  return value;
}

int64_t profile_attributes_integer(IDeckLinkProfileAttributes *attrs,
                                   IntegerAttributeId id) {
  int64_t value;
  if (attrs->GetInt(integer_attribute_id(id), &value) != S_OK) {
    throw std::runtime_error("IDeckLinkProfileAttributes::GetInt failed.");
  }
  return value;
}

double profile_attributes_float(IDeckLinkProfileAttributes *attrs,
                                FloatAttributeId id) {
  double value;
  if (attrs->GetFloat(float_attribute_id(id), &value) != S_OK) {
    throw std::runtime_error("IDeckLinkProfileAttributes::GetFloat failed.");
  }
  return value;
}

rust::String profile_attributes_string(IDeckLinkProfileAttributes *attrs,
                                       StringAttributeId id) {
  const char *value;
  if (attrs->GetString(string_attribute_id(id), &value) != S_OK) {
    throw std::runtime_error("IDeckLinkProfileAttributes::GetString failed.");
  }
  return rust::String(value);
}

void profile_attributes_release(IDeckLinkProfileAttributes *attrs) noexcept {
  attrs->Release();
}

//
// IDeckLinkInput
//

DisplayMode
input_supports_video_mode(IDeckLinkInput *input, VideoConnection conn,
                          DisplayMode mode, PixelFormat pixel_format,
                          VideoInputConversionMode conversion_mode,
                          SupportedVideoModeFlags supported_mode_flags) {
  BMDDisplayMode actual_mode;
  bool supported; // TODO
  auto result = input->DoesSupportVideoMode(
      from_video_connection(conn), from_display_mode(mode),
      from_pixel_format(pixel_format),
      from_video_input_conversion_mode(conversion_mode),
      from_supported_video_mode_flags(supported_mode_flags), &actual_mode,
      &supported);
  if (result != S_OK) {
    throw std::runtime_error("IDeckLinkInput::DoesSupportVideoMode failed.");
  }
  return into_display_mode(actual_mode);
}

void input_enable_video_output(IDeckLinkInput *input, DisplayMode mode,
                               PixelFormat format, VideoInputFlags flags) {
  auto result = input->EnableVideoInput(from_display_mode(mode),
                                        from_pixel_format(format),
                                        from_video_input_flags(flags));
  if (result != S_OK) {
    throw std::runtime_error("IDeckLinkInput::EnableVideoInput failed.");
  }
}

void input_enable_audio_output(IDeckLinkInput *input, uint32_t sample_rate,
                               AudioSampleType sample_type, uint32_t channels) {
  auto result = input->EnableAudioInput(
      sample_rate, static_cast<uint32_t>(sample_type), channels);
  if (result != S_OK) {
    throw std::runtime_error("IDeckLinkInput::EnableAudioInput failed.");
  }
}

void input_set_callback(IDeckLinkInput *input, rust::Box<DynInputCallback> cb) {
  auto wrapper = new InputCallbackWrapper(std::move(cb));
  auto result =
      input->SetCallback(static_cast<IDeckLinkInputCallback *>(wrapper));
  if (result != S_OK) {
    throw std::runtime_error("IDeckLinkInput::SetCallback failed.");
  }
}

void input_start_streams(IDeckLinkInput *input) {
  auto result = input->StartStreams();
  if (result != S_OK) {
    throw std::runtime_error("IDeckLinkInput::StartStreams failed.");
  }
}

void input_stop_streams(IDeckLinkInput *input) {
  auto result = input->StopStreams();
  if (result != S_OK) {
    throw std::runtime_error("IDeckLinkInput::StopStreams failed.");
  }
}

void input_release(IDeckLinkInput *input) noexcept { input->Release(); }

//
// IDeckLinkVideoInputFrame
//

long video_input_frame_width(IDeckLinkVideoInputFrame *input) noexcept {
  return input->GetWidth();
}

long video_input_frame_height(IDeckLinkVideoInputFrame *input) noexcept {
  return input->GetHeight();
}

long video_input_frame_row_bytes(IDeckLinkVideoInputFrame *input) noexcept {
  return input->GetRowBytes();
}

uint8_t *video_input_frame_bytes(IDeckLinkVideoInputFrame *input) {
  void *buffer = nullptr;
  auto result = input->GetBytes(&buffer);
  if (result != S_OK) {
    throw std::runtime_error("IDeckLinkVideoInputFrame::GetBytes failed.");
  }
  return reinterpret_cast<uint8_t*>(buffer);
}
