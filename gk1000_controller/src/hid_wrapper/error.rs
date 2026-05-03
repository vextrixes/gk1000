use crate::hid_wrapper::error::HidWrapperError::{HidApiError, NoHidDeviceError};
use hidapi::HidError;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum HidWrapperError {
    #[allow(dead_code)]
    HidApiError(HidError),
    NoHidDeviceError,
}

impl Display for HidWrapperError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HidApiError(err) => { write!(f, "Hid Api Error: {err}") }
            NoHidDeviceError => { write!(f, "No Hid Device Found") }
        }
    } //TODO
}

impl Error for HidWrapperError {}