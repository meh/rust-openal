pub mod extension;

use std::ffi::CString;
use std::ptr;
use std::marker::PhantomData;
use std::ffi::CStr;
use std::str::from_utf8_unchecked;

use libc::strlen;
use ffi::*;
use ::Error;

pub struct Device<'a> {
	ptr: *mut ALCdevice,

	_own: bool,
	_marker: PhantomData<&'a ()>,
}

impl<'a> Device<'a> {
	pub unsafe fn wrap(ptr: *mut ALCdevice) -> Self {
		Device { ptr: ptr, _own: false, _marker: PhantomData }
	}

	pub unsafe fn as_ptr(&self) -> *const ALCdevice {
		self.ptr as *const _
	}

	pub unsafe fn as_mut_ptr(&mut self) -> *mut ALCdevice {
		self.ptr
	}
}

impl<'a> Device<'a> {
	pub fn default() -> Result<Self, Error> {
		unsafe {
			let ptr = alcOpenDevice(ptr::null());

			if ptr.is_null() {
				Err(Error::InvalidName)
			}
			else {
				Ok(Device { _own: true, ..Device::wrap(ptr) })
			}
		}
	}

	pub fn open(name: &str) -> Result<Self, Error> {
		unsafe {
			let ptr = alcOpenDevice(CString::new(name.as_bytes()).unwrap().as_ptr());

			if ptr.is_null() {
				Err(Error::InvalidName)
			}
			else {
				Ok(Device { _own: true, ..Device::wrap(ptr) })
			}
		}
	}
}

impl<'a> Drop for Device<'a> {
	fn drop(&mut self) {
		unsafe {
			if self._own {
				if cfg!(debug_assertions) {
					if alcCloseDevice(self.as_mut_ptr()) != ALC_TRUE {
						if cfg!(debug_assertions) {
							panic!("device still in use");
						}
					}
				}
				else {
					alcCloseDevice(self.as_mut_ptr());
				}
			}
		}
	}
}

pub fn names() -> Vec<&'static str> {
	let mut result = Vec::new();

	unsafe {
		if extension::is_supported(&Device::wrap(ptr::null_mut()), "ALC_ENUMERATION_EXT") {
			let mut ptr = alcGetString(ptr::null(), ALC_DEVICE_SPECIFIER);

			while *ptr != 0 {
				result.push(from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()));

				ptr = ptr.offset(strlen(ptr) as isize + 1);
			}
		}
	}

	result
}
