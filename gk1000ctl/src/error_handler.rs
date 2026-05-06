use gk1000_controller::HidWrapperError;
use std::process::exit;

pub fn error_handler(err: HidWrapperError) -> () {
    match err {
        HidWrapperError::NoHidDeviceError => {
            println!("NoHidDeviceError: Is the keyboard plugged in?");
            exit(libc::ENODEV)
        }
        HidWrapperError::HidApiError(err) => {
            println!("HidApiError: {:?}", err);
            exit(libc::EIO)
        }
    }
}
