/*!
Helpers related to `Capture`.
*/

mod capture;
pub use self::capture::Capture;

use std::ptr;

use ffi::*;
use {Error, Sample, extension};

/// Opens the default capture device.
pub fn default<T: Sample>(channels: u16, rate: u32, size: usize) -> Result<Capture<T>, Error> {
	Capture::<T>::default(channels, rate, size)
}

/// Opens the named output device.
pub fn open<T: Sample>(name: &str, channels: u16, rate: u32, size: usize) -> Result<Capture<T>, Error> {
	Capture::<T>::open(name, channels, rate, size)
}

/// Gets a list of available capture device names.
pub fn devices() -> Vec<&'static str> {
	use std::ffi::CStr;
	use std::str::from_utf8_unchecked;
	use libc::strlen;

	let mut result = Vec::new();

	unsafe {
		if extension::device::is_supported("ALC_ENUMERATION_EXT") {
			let mut ptr = alcGetString(ptr::null(), ALC_CAPTURE_DEVICE_SPECIFIER);

			while *ptr != 0 {
				result.push(from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()));

				ptr = ptr.offset(strlen(ptr) as isize + 1);
			}
		}
	}

	result
}
