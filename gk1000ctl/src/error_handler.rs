use gk1000_controller::HidWrapperError;
use std::process::exit;

pub fn error_handler(err: HidWrapperError) -> () {
    match err {
        HidWrapperError::NoHidDeviceError => {
            eprintln!("NoHidDeviceError: Is the keyboard plugged in?");
            exit(libc::ENODEV)
        }
        HidWrapperError::HidApiError(err) => {
            eprintln!("HidApiError: {:?}", err);
            exit(libc::EIO)
        }
    }
}
