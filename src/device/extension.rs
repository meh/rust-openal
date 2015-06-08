use std::ffi::CString;

use ffi::*;
use ::Device;

pub fn is_supported(device: &Device, name: &str) -> bool {
	unsafe {
		alcIsExtensionPresent(device.as_ptr(), CString::new(name).unwrap().as_ptr()) == AL_TRUE
	}
}
