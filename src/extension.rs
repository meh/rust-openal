/*!
Helpers for extension support.
*/

use std::ffi::CString;

use ffi::*;

/// Checks if the given extension is supported.
pub fn is_supported(name: &str) -> bool {
	unsafe {
		alIsExtensionPresent(CString::new(name).unwrap().as_ptr()) == AL_TRUE
	}
}

/// Helpers for device related extension support.
pub mod device {
	use std::ptr;
	use std::ffi::CString;

	use ffi::*;
	use ::traits::Device;

	/// Checks if the given extension is supported.
	pub fn is_supported(name: &str) -> bool {
		unsafe {
			alcIsExtensionPresent(ptr::null(), CString::new(name).unwrap().as_ptr()) == AL_TRUE
		}
	}

	/// Checks if the given extension is supported by the given device.
	pub fn is_supported_by<T: Device>(device: &T, name: &str) -> bool {
		unsafe {
			alcIsExtensionPresent(device.as_ptr(), CString::new(name).unwrap().as_ptr()) == AL_TRUE
		}
	}
}
