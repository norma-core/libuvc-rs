use std::ffi::CStr;
use std::fmt;

/// Result type of functions in this crate
pub type Result<T> = std::result::Result<T, Error>;

/// Error codes from `libusb`
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Error {
    Success,
    Access,
    Busy,
    CallbackExists,
    Interrupted,
    InvalidDevice,
    InvalidMode,
    InvalidParam,
    IO,
    NotFound,
    NotSupported,
    NoDevice,
    NoMem,
    Other,
    Overflow,
    Pipe,
    Timeout,
    Unknown(norm_uvc_sys::uvc_error_t),
}

impl From<norm_uvc_sys::uvc_error_t> for Error {
    fn from(code: norm_uvc_sys::uvc_error_t) -> Self {
        match code {
            norm_uvc_sys::uvc_error_UVC_SUCCESS => Error::Success,
            norm_uvc_sys::uvc_error_UVC_ERROR_ACCESS => Error::Access,
            norm_uvc_sys::uvc_error_UVC_ERROR_BUSY => Error::Busy,
            norm_uvc_sys::uvc_error_UVC_ERROR_CALLBACK_EXISTS => Error::CallbackExists,
            norm_uvc_sys::uvc_error_UVC_ERROR_INTERRUPTED => Error::Interrupted,
            norm_uvc_sys::uvc_error_UVC_ERROR_INVALID_DEVICE => Error::InvalidDevice,
            norm_uvc_sys::uvc_error_UVC_ERROR_INVALID_MODE => Error::InvalidMode,
            norm_uvc_sys::uvc_error_UVC_ERROR_INVALID_PARAM => Error::InvalidParam,
            norm_uvc_sys::uvc_error_UVC_ERROR_IO => Error::IO,
            norm_uvc_sys::uvc_error_UVC_ERROR_NOT_FOUND => Error::NotFound,
            norm_uvc_sys::uvc_error_UVC_ERROR_NOT_SUPPORTED => Error::NotSupported,
            norm_uvc_sys::uvc_error_UVC_ERROR_NO_DEVICE => Error::NoDevice,
            norm_uvc_sys::uvc_error_UVC_ERROR_NO_MEM => Error::NoMem,
            norm_uvc_sys::uvc_error_UVC_ERROR_OTHER => Error::Other,
            norm_uvc_sys::uvc_error_UVC_ERROR_OVERFLOW => Error::Overflow,
            norm_uvc_sys::uvc_error_UVC_ERROR_PIPE => Error::Pipe,
            norm_uvc_sys::uvc_error_UVC_ERROR_TIMEOUT => Error::Timeout,
            x => Error::Unknown(x),
        }
    }
}

impl Into<norm_uvc_sys::uvc_error_t> for Error {
    fn into(self) -> norm_uvc_sys::uvc_error_t {
        match self {
            Error::Success => norm_uvc_sys::uvc_error_UVC_SUCCESS,
            Error::Access => norm_uvc_sys::uvc_error_UVC_ERROR_ACCESS,
            Error::Busy => norm_uvc_sys::uvc_error_UVC_ERROR_BUSY,
            Error::CallbackExists => norm_uvc_sys::uvc_error_UVC_ERROR_CALLBACK_EXISTS,
            Error::Interrupted => norm_uvc_sys::uvc_error_UVC_ERROR_INTERRUPTED,
            Error::InvalidDevice => norm_uvc_sys::uvc_error_UVC_ERROR_INVALID_DEVICE,
            Error::InvalidMode => norm_uvc_sys::uvc_error_UVC_ERROR_INVALID_MODE,
            Error::InvalidParam => norm_uvc_sys::uvc_error_UVC_ERROR_INVALID_PARAM,
            Error::IO => norm_uvc_sys::uvc_error_UVC_ERROR_IO,
            Error::NotFound => norm_uvc_sys::uvc_error_UVC_ERROR_NOT_FOUND,
            Error::NotSupported => norm_uvc_sys::uvc_error_UVC_ERROR_NOT_SUPPORTED,
            Error::NoDevice => norm_uvc_sys::uvc_error_UVC_ERROR_NO_DEVICE,
            Error::NoMem => norm_uvc_sys::uvc_error_UVC_ERROR_NO_MEM,
            Error::Other => norm_uvc_sys::uvc_error_UVC_ERROR_OTHER,
            Error::Overflow => norm_uvc_sys::uvc_error_UVC_ERROR_OVERFLOW,
            Error::Pipe => norm_uvc_sys::uvc_error_UVC_ERROR_PIPE,
            Error::Timeout => norm_uvc_sys::uvc_error_UVC_ERROR_TIMEOUT,
            Error::Unknown(x) => x,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let strerror = unsafe { norm_uvc_sys::uvc_strerror((*self).into()) };
        if strerror.is_null() {
            return write!(f, "Unknown error");
        }
        let strerr = unsafe { CStr::from_ptr(strerror) }.to_str().unwrap();
        write!(f, "{}", strerr)
    }
}
impl std::error::Error for Error {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }
}
