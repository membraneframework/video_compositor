use crate::DeckLinkError;

use super::ffi;

pub struct DeckLinkConfiguration(pub(super) *mut ffi::IDeckLinkConfiguration);

impl DeckLinkConfiguration {
    pub fn get_flag(&self, id: ffi::FlagConfigurationId) -> Result<bool, DeckLinkError> {
        unsafe { Ok(ffi::configuration_flag(self.0, id)?) }
    }
    pub fn get_integer(&self, id: ffi::IntegerConfigurationId) -> Result<i64, DeckLinkError> {
        unsafe { Ok(ffi::configuration_integer(self.0, id)?) }
    }
    pub fn get_float(&self, id: ffi::FloatConfigurationId) -> Result<f64, DeckLinkError> {
        unsafe { Ok(ffi::configuration_float(self.0, id)?) }
    }
    pub fn get_string(&self, id: ffi::StringConfigurationId) -> Result<String, DeckLinkError> {
        unsafe { Ok(ffi::configuration_string(self.0, id)?) }
    }
    pub fn set_flag(
        &mut self,
        id: ffi::FlagConfigurationId,
        value: bool,
    ) -> Result<(), DeckLinkError> {
        unsafe { Ok(ffi::configuration_set_flag(self.0, id, value)?) }
    }
    pub fn set_integer(
        &mut self,
        id: ffi::IntegerConfigurationId,
        value: i64,
    ) -> Result<(), DeckLinkError> {
        unsafe { Ok(ffi::configuration_set_integer(self.0, id, value)?) }
    }
    pub fn set_float(
        &mut self,
        id: ffi::FloatConfigurationId,
        value: f64,
    ) -> Result<(), DeckLinkError> {
        unsafe { Ok(ffi::configuration_set_float(self.0, id, value)?) }
    }
    pub fn set_string(
        &mut self,
        id: ffi::StringConfigurationId,
        value: String,
    ) -> Result<(), DeckLinkError> {
        unsafe { Ok(ffi::configuration_set_string(self.0, id, value)?) }
    }
}

impl Drop for DeckLinkConfiguration {
    fn drop(&mut self) {
        unsafe { ffi::configuration_release(self.0) }
    }
}
