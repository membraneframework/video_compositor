use crate::{
    api::{
        device::DeckLinkConfiguration, into_video_io_support, profile::ProfileAttributes,
        VideoIOSupport,
    },
    enums::ffi::{FlagAttributeId, IntegerAttributeId, StringAttributeId},
    DeckLink, DeckLinkError,
};

#[derive(Debug)]
pub struct DeckLinkInfo {
    pub current_profile: ProfileAttributesInfo,
    pub profiles: Vec<ProfileInfo>,
    pub configuration: ConfigurationInfo,
}

impl DeckLink {
    pub fn info(&self) -> Result<DeckLinkInfo, DeckLinkError> {
        let profiles = match self.profile_manager()? {
            Some(manager) => manager
                .profiles()?
                .into_iter()
                .map(|profile| -> Result<ProfileInfo, DeckLinkError> {
                    Ok(ProfileInfo {
                        is_active: profile.is_active()?,
                        attributes: profile.attributes()?.info()?,
                    })
                })
                .collect::<Result<Vec<_>, _>>()?,
            None => vec![],
        };
        let current_profile = self.profile_attributes()?.info()?;
        let configuration = self.configuration()?.info()?;
        Ok(DeckLinkInfo {
            current_profile,
            profiles,
            configuration,
        })
    }
}

#[derive(Debug)]
pub struct ProfileInfo {
    pub is_active: bool,
    pub attributes: ProfileAttributesInfo,
}

#[derive(Debug)]
pub struct ProfileAttributesInfo {
    pub video_io_support: VideoIOSupport,
    pub model_name: String,
    pub vendor_name: String,
    pub display_name: String,
    pub device_handle: String,
    pub ethernet_mac_address: String,
    pub serial_port_device_name: String,

    pub profile_id: i64,
    pub max_audio_channels: i64,
    pub max_hdmi_audio_channels: i64,
    pub number_of_subdevices: i64,
    pub subdevice_index: i64,
    pub persistent_id: i64,
    pub device_group_id: i64,
    pub topological_id: i64,

    pub supports_input_format_detection: bool,
    pub has_serial_port: bool,
}

impl ProfileAttributes {
    pub fn info(&self) -> Result<ProfileAttributesInfo, DeckLinkError> {
        Ok(ProfileAttributesInfo {
            video_io_support: into_video_io_support(
                self.get_integer(IntegerAttributeId::VideoIOSupport)?,
            ),
            model_name: self.get_string(StringAttributeId::ModelName)?,
            vendor_name: self.get_string(StringAttributeId::VendorName)?,
            display_name: self.get_string(StringAttributeId::DisplayName)?,
            device_handle: self.get_string(StringAttributeId::DeviceHandle)?,
            ethernet_mac_address: self.get_string(StringAttributeId::EthernetMACAddress)?,
            // TODO: free
            serial_port_device_name: self.get_string(StringAttributeId::SerialPortDeviceName)?,

            profile_id: self.get_integer(IntegerAttributeId::ProfileID)?,
            max_audio_channels: self.get_integer(IntegerAttributeId::MaximumAudioChannels)?,
            max_hdmi_audio_channels: self
                .get_integer(IntegerAttributeId::MaximumHDMIAudioChannels)?,
            number_of_subdevices: self.get_integer(IntegerAttributeId::NumberOfSubDevices)?,
            subdevice_index: self.get_integer(IntegerAttributeId::SubDeviceIndex)?,
            persistent_id: self.get_integer(IntegerAttributeId::PersistentID)?,
            device_group_id: self.get_integer(IntegerAttributeId::DeviceGroupID)?,
            topological_id: self.get_integer(IntegerAttributeId::TopologicalID)?,

            supports_input_format_detection: self
                .get_flag(FlagAttributeId::SupportsInputFormatDetection)?,
            has_serial_port: self.get_flag(FlagAttributeId::HasSerialPort)?,
        })
    }
}

#[derive(Debug)]
pub struct ConfigurationInfo {}

impl DeckLinkConfiguration {
    pub fn info(&self) -> Result<ConfigurationInfo, DeckLinkError> {
        Ok(ConfigurationInfo {})
    }
}
