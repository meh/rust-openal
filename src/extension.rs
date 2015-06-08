use std::ffi::CString;

use ffi::*;

pub fn is_supported(name: &str) -> bool {
	unsafe {
		alIsExtensionPresent(CString::new(name).unwrap().as_ptr()) == AL_TRUE
	}
}
