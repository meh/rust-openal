pub mod listener;
pub use self::listener::Listener;

pub mod current;
pub use self::current::Current;

use std::ptr;

use ffi::*;
use {Error, extension};

pub fn default<'a>() -> Result<Listener<'a>, Error> {
	Listener::default()
}

pub fn open<'a>(name: &str) -> Result<Listener<'a>, Error> {
	Listener::open(name)
}

pub fn devices() -> Vec<&'static str> {
	use std::ffi::CStr;
	use std::str::from_utf8_unchecked;
	use libc::strlen;

	let mut result = Vec::new();

	unsafe {
		if extension::device::is_supported("ALC_ENUMERATION_EXT") {
			let mut ptr = alcGetString(ptr::null(), ALC_DEVICE_SPECIFIER);

			while *ptr != 0 {
				result.push(from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()));

				ptr = ptr.offset(strlen(ptr) as isize + 1);
			}
		}
	}

	result
}
