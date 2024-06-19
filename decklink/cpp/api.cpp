#include "api.h"
#include "callback.h"
#include "decklink/decklink_sdk/include/DeckLinkAPI.h"
#include "decklink/decklink_sdk/include/DeckLinkAPITypes.h"
#include "enums.h"

#include "decklink/src/api.rs.h"
#include "decklink/src/enums.rs.h"
#include <cstdint>
#include <format>
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

IDeckLinkProfileManager *decklink_profile_manager(IDeckLink *decklink) {
  IDeckLinkProfileManager *manager = nullptr;

  HRESULT result =
      decklink->QueryInterface(IID_IDeckLinkProfileManager, (void **)&manager);
  if (result == E_NOINTERFACE) {
    return nullptr;
  }
  if (result != S_OK) {
    throw std::runtime_error(
        "IDeckLink::QueryInterface IID_IDeckLinkProfileManager failed.");
  }
  return manager;
}

IDeckLinkConfiguration *decklink_configuration(IDeckLink *decklink) {
  IDeckLinkConfiguration *conf = nullptr;

  HRESULT result =
      decklink->QueryInterface(IID_IDeckLinkConfiguration, (void **)&conf);
  if (result != S_OK) {
    throw std::runtime_error(
        "IDeckLink::QueryInterface IID_IDeckLinkConfiguration failed.");
  }
  return conf;
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
  auto result = attrs->GetInt(integer_attribute_id(id), &value);
  if (result == E_INVALIDARG) {
    return 0;
  }
  if (result != S_OK) {
    throw std::runtime_error(
        std::format("IDeckLinkProfileAttributes::GetInt failed. {:#x}",
                    static_cast<unsigned int>(result)));
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
  auto result = attrs->GetString(string_attribute_id(id), &value);
  if (result == E_INVALIDARG || result == E_NOTIMPL) {
    return rust::String("");
  }
  if (result != S_OK) {
    throw std::runtime_error(
        std::format("IDeckLinkProfileAttributes::GetString failed. {:#x}",
                    static_cast<unsigned int>(result)));
  }
  return rust::String(value);
}

void profile_attributes_release(IDeckLinkProfileAttributes *attrs) noexcept {
  attrs->Release();
}

//
// IDeckLinkInput
//

DisplayModeType
input_supports_video_mode(IDeckLinkInput *input, VideoConnection conn,
                          DisplayModeType mode, PixelFormat pixel_format,
                          VideoInputConversionMode conversion_mode,
                          SupportedVideoModeFlags supported_mode_flags) {
  BMDDisplayMode actual_mode;
  bool supported;
  auto result = input->DoesSupportVideoMode(
      from_video_connection(conn), from_display_mode_type(mode),
      from_pixel_format(pixel_format),
      from_video_input_conversion_mode(conversion_mode),
      from_supported_video_mode_flags(supported_mode_flags), &actual_mode,
      &supported);
  if (result != S_OK) {
    throw std::runtime_error(
        std::format("IDeckLinkInput::DoesSupportVideoMode failed. ({:#x})",
                    static_cast<unsigned int>(result)));
  }
  if (!supported) {
    throw std::runtime_error(
        "IDeckLinkInput::DoesSupportVideoMode: Device does not "
        "support selected params");
  }
  return into_display_mode_type(actual_mode);
}

void input_enable_video(IDeckLinkInput *input, DisplayModeType mode,
                        PixelFormat format, VideoInputFlags flags) {
  auto result = input->EnableVideoInput(from_display_mode_type(mode),
                                        from_pixel_format(format),
                                        from_video_input_flags(flags));
  if (result != S_OK) {
    throw std::runtime_error("IDeckLinkInput::EnableVideoInput failed.");
  }
}

void input_enable_audio(IDeckLinkInput *input, uint32_t sample_rate,
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

void input_pause_streams(IDeckLinkInput *input) {
  auto result = input->PauseStreams();
  if (result != S_OK) {
    throw std::runtime_error("IDeckLinkInput::PauseStreams failed.");
  }
}

void input_flush_streams(IDeckLinkInput *input) {
  auto result = input->FlushStreams();
  if (result != S_OK) {
    throw std::runtime_error("IDeckLinkInput::FlushStreams failed.");
  }
}

void input_release(IDeckLinkInput *input) noexcept { input->Release(); }

//
// IDeckLinkProfileManager
//

rust::Vec<IDeckLinkProfilePtr>
profile_manager_profiles(IDeckLinkProfileManager *manager) {
  IDeckLinkProfileIterator *profile_iterator;
  if (manager->GetProfiles(&profile_iterator) != S_OK) {
    throw std::runtime_error("IDeckLinkProfileManager::GetProfiles failed.");
  }

  rust::Vec<IDeckLinkProfilePtr> profiles;
  while (true) {
    IDeckLinkProfile *profile;
    HRESULT result = profile_iterator->Next(&profile);
    if (result != S_OK) {
      profile_iterator->Release();
      return profiles;
    }
    profiles.push_back(IDeckLinkProfilePtr{profile});
  }
}

void profile_manager_release(IDeckLinkProfileManager *manager) noexcept {
  manager->Release();
}

//
// IDeckLinkProfile
//

IDeckLinkProfileAttributes *
profile_profile_attributes(IDeckLinkProfile *profile) {
  IDeckLinkProfileAttributes *attributes = nullptr;

  HRESULT result = profile->QueryInterface(IID_IDeckLinkProfileAttributes,
                                           (void **)&attributes);
  if (result != S_OK) {
    throw std::runtime_error("IDeckLinkProfile::QueryInterface "
                             "IID_IDeckLinkProfileAttributes failed.");
  }
  return attributes;
}

bool profile_is_active(IDeckLinkProfile *profile) {
  bool is_active;
  if (profile->IsActive(&is_active) != S_OK) {
    throw std::runtime_error("IDeckLinkProfile::IsActive failed.");
  }
  return is_active;
}

void profile_release(IDeckLinkProfile *profile) noexcept { profile->Release(); }

//
// IDeckLinkConfiguration
//

bool configuration_flag(IDeckLinkConfiguration *conf, FlagConfigurationId id) {
  bool value;
  if (conf->GetFlag(flag_configuration_id(id), &value) != S_OK) {
    throw std::runtime_error("IDeckLinkConfigurations::GetFlag failed.");
  }
  return value;
}

int64_t configuration_integer(IDeckLinkConfiguration *conf,
                              IntegerConfigurationId id) {
  int64_t value;
  if (conf->GetInt(integer_configuration_id(id), &value) != S_OK) {
    throw std::runtime_error("IDeckLinkConfigurations::GetInt failed.");
  }
  return value;
}

double configuration_float(IDeckLinkConfiguration *conf,
                           FloatConfigurationId id) {
  double value;
  if (conf->GetFloat(float_configuration_id(id), &value) != S_OK) {
    throw std::runtime_error("IDeckLinkConfigurations::GetFloat failed.");
  }
  return value;
}

rust::String configuration_string(IDeckLinkConfiguration *conf,
                                  StringConfigurationId id) {
  const char *value;
  if (conf->GetString(string_configuration_id(id), &value) != S_OK) {
    throw std::runtime_error("IDeckLinkConfigurations::GetString failed.");
  }
  return rust::String(value);
}

void configuration_set_flag(IDeckLinkConfiguration *conf,
                            FlagConfigurationId id, bool value) {
  if (conf->SetFlag(flag_configuration_id(id), value) != S_OK) {
    throw std::runtime_error("IDeckLinkConfigurations::SetFlag failed.");
  }
}

void configuration_set_integer(IDeckLinkConfiguration *conf,
                               IntegerConfigurationId id, int64_t value) {
  if (conf->SetInt(integer_configuration_id(id), value) != S_OK) {
    throw std::runtime_error("IDeckLinkConfigurations::SetInt failed.");
  }
}
void configuration_set_float(IDeckLinkConfiguration *conf,
                             FloatConfigurationId id, double value) {
  if (conf->SetFloat(float_configuration_id(id), value) != S_OK) {
    throw std::runtime_error("IDeckLinkConfigurations::SetFloat failed.");
  }
}
void configuration_set_string(IDeckLinkConfiguration *conf,
                              StringConfigurationId id, rust::String value) {
  if (conf->SetString(string_configuration_id(id), value.c_str()) != S_OK) {
    throw std::runtime_error("IDeckLinkConfigurations::SetString failed.");
  }
}

void configuration_release(IDeckLinkConfiguration *conf) noexcept {
  conf->Release();
}

//
// IDeckLinkVideoInputFrame
//

long video_input_frame_width(IDeckLinkVideoInputFrame *frame) noexcept {
  return frame->GetWidth();
}

long video_input_frame_height(IDeckLinkVideoInputFrame *frame) noexcept {
  return frame->GetHeight();
}

long video_input_frame_row_bytes(IDeckLinkVideoInputFrame *frame) noexcept {
  return frame->GetRowBytes();
}

uint8_t *video_input_frame_bytes(IDeckLinkVideoInputFrame *frame) {
  void *buffer = nullptr;
  auto result = frame->GetBytes(&buffer);
  if (result != S_OK) {
    throw std::runtime_error("IDeckLinkVideoInputFrame::GetBytes failed.");
  }
  return reinterpret_cast<uint8_t *>(buffer);
}

BMDTimeValue video_input_frame_stream_time(IDeckLinkVideoInputFrame *frame,
                                           BMDTimeScale time_scale) {
  BMDTimeValue time;
  BMDTimeValue duration;
  if (frame->GetStreamTime(&time, &duration, time_scale) != S_OK) {
    throw std::runtime_error("IDeckLinkVideoInputFrame::GetStreamTime failed.");
  }
  return time;
}

//
// IDeckLinkAudioInputPacket
//

uint8_t *audio_input_packet_bytes(IDeckLinkAudioInputPacket *packet) {
  void *buffer = nullptr;
  if (packet->GetBytes(&buffer) != S_OK) {
    throw std::runtime_error("IDeckLinkAudioInputPacket::GetBytes failed.");
  }
  return reinterpret_cast<uint8_t *>(buffer);
}

int64_t
audio_input_packet_sample_count(IDeckLinkAudioInputPacket *packet) noexcept {
  return packet->GetSampleFrameCount();
}

BMDTimeValue audio_input_packet_packet_time(IDeckLinkAudioInputPacket *packet,
                                            BMDTimeScale time_scale) {
  BMDTimeValue time;
  if (packet->GetPacketTime(&time, time_scale) != S_OK) {
    throw std::runtime_error(
        "IDeckLinkAudioInputPacket::GetPacketTime failed.");
  }
  return time;
}

//
// IDeckLinkDisplayMode
//

int64_t display_mode_width(IDeckLinkDisplayMode *mode) noexcept {
  return mode->GetWidth();
}

int64_t display_mode_height(IDeckLinkDisplayMode *mode) noexcept {
  return mode->GetHeight();
}

rust::String display_mode_name(IDeckLinkDisplayMode *mode) {
  const char *name;
  if (mode->GetName(&name) != S_OK) {
    throw std::runtime_error("IDeckLinkDisplayMode::GetName failed.");
  }
  auto result = rust::String(name);
  free(const_cast<char *>(name));
  return result;
}

DisplayModeType display_mode_display_mode_type(IDeckLinkDisplayMode *mode) {
  return into_display_mode_type(mode->GetDisplayMode());
}

Ratio display_mode_frame_rate(IDeckLinkDisplayMode *mode) {
  BMDTimeValue num;
  BMDTimeScale den;
  if (mode->GetFrameRate(&num, &den) != S_OK) {
    throw std::runtime_error("IDeckLinkDisplayMode::GetFrameRate failed.");
  }
  return Ratio{num, den};
}

void display_mode_release(IDeckLinkDisplayMode *mode) noexcept {
  mode->Release();
}
