extern crate hidapi;

use crate::hid_wrapper::HidWrapperError::HidApiError;
use hidapi::{HidApi, HidDevice, HidError};

const VENDOR_ID: u16 = 0x0c45;
const PRODUCT_ID: u16 = 0x8006;
const MAX_SEND_REPORT_LEN: usize = 65;
const GET_REPORT_REPORT: [u8; MAX_SEND_REPORT_LEN] = [0u8; MAX_SEND_REPORT_LEN];

pub enum HidWrapperError {
    #[allow(dead_code)]
    HidApiError(HidError),
}

pub struct HidWrapper {
    hid_api: HidApi,
    hid_device: Option<HidDevice>,
}

impl Default for HidWrapper {
    fn default() -> Self {
        match Self::new() {
            Ok(hid_wrapper) => hid_wrapper,
            Err(_) => panic!("TODO err"),
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

    pub fn open_device(&mut self) {
        self.hid_device = match self.hid_api.open(VENDOR_ID, PRODUCT_ID) {
            Ok(hid_device) => Some(hid_device),
            Err(_err) => panic!("TODO err"),
        }
    }

    pub fn close_device(&mut self) {
        self.hid_device = None;
    }

    pub fn send_report(&mut self, data: &[u8]) {
        match &self.hid_device {
            Some(hid_device) => {
                let mut out = [0; MAX_SEND_REPORT_LEN];
                out[1..1 + data.len()].copy_from_slice(&data);
                let _ = hid_device.send_feature_report(&out);
            }
            None => panic!("TODO err"),
        };
    }
    pub fn get_report(&mut self) -> usize {
        match &self.hid_device {
            Some(hid_device) => {
                let mut request = GET_REPORT_REPORT.clone();
                request[1] = 64;
                match hid_device.get_feature_report(&mut request) {
                    Ok(usize) => usize,
                    Err(_err) => panic!("TODO err"),
                }
            }
            None => panic!("TODO err"),
        }
    }
}
