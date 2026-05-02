extern crate hidapi;

use std::fmt::Display;

use hidapi::{HidApi, HidDevice, HidError};
use HidWrapperError::{HidApiError, NoHidDeviceError};

const VENDOR_ID: u16 = 0x0c45;
const PRODUCT_ID: u16 = 0x8006;
const MAX_SEND_REPORT_LEN: usize = 65;
const GET_REPORT_REPORT: [u8; MAX_SEND_REPORT_LEN] = [0u8; MAX_SEND_REPORT_LEN];

pub enum HidWrapperError {
    #[allow(dead_code)]
    HidApiError(HidError),
    NoHidDeviceError(),
}

impl Display for HidWrapperError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "se")
    } //TODO
}

pub struct HidWrapper {
    hid_api: HidApi,
    hid_device: Option<HidDevice>,
}

impl Default for HidWrapper {
    fn default() -> Self {
        match Self::new() {
            Ok(hid_wrapper) => hid_wrapper,
            Err(err) => panic!("hid wrapper Default() failed to create new HidWrapper: {err}"),
        }
    }
}

impl HidWrapper {
    pub fn new() -> Result<HidWrapper, HidWrapperError> {
        Ok(HidWrapper {
            hid_api: match HidApi::new() {
                Ok(hidapi) => hidapi,
                Err(err) => return Err(HidApiError(err)),
            },
            hid_device: None,
        })
    }

    /// Opens a device
    ///
    /// Errors
    /// ```HidWrapperError::HidApiError```
    pub fn open_device(&mut self) -> Result<(), HidWrapperError> {
        self.hid_device = match self.hid_api.open(VENDOR_ID, PRODUCT_ID) {
            Ok(hid_device) => Some(hid_device),
            Err(err) => return Err(HidApiError(err)),
        };
        Ok(())
    }

    /// Drops the device
    ///
    pub fn close_device(&mut self) {
        self.hid_device = None;
    }

    /// Sends a set report (65 bytes len)
    ///
    /// Errors
    /// ```HidWrapperError::HidApiError```
    /// ```HidWrapperError::NoHidDeviceError```
    pub fn send_report(&mut self, data: &[u8]) -> Result<(), HidWrapperError> {
        match &self.hid_device {
            Some(hid_device) => {
                let mut out = [0; MAX_SEND_REPORT_LEN];
                out[1..1 + data.len()].copy_from_slice(data);
                match hid_device.send_feature_report(&out) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(HidApiError(err))
                }
            }
            None => Err(NoHidDeviceError())
        }
    }

    /// Sends a get report (64 bytes len)
    ///
    /// Errors
    /// ```HidWrapperError::HidApiError```
    /// ```HidWrapperError::NoHidDeviceError```
    pub fn get_report(&mut self) -> Result<usize, HidWrapperError> {
        match &self.hid_device {
            Some(hid_device) => {
                let mut request = GET_REPORT_REPORT;
                request[1] = 64;
                match hid_device.get_feature_report(&mut request) {
                    Ok(usize) => Ok(usize),
                    Err(err) => Err(HidApiError(err)),
                }
            }
            None => Err(NoHidDeviceError())
        }
    }
}
