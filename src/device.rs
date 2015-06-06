use std::ffi::CString;
use std::ptr;
use std::marker::PhantomData;

use ffi::*;
use ::{Error, error};

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
	pub fn open(name: Option<&str>) -> Result<Self, Error> {
		unsafe {
			let ptr = if let Some(name) = name {
				alcOpenDevice(CString::new(name.as_bytes()).unwrap().as_ptr())
			}
			else {
				alcOpenDevice(ptr::null())
			};

			if ptr.is_null() {
				Err(Error::InvalidName)
			}
			else {
				Ok(Device { _own: true, ..Device::wrap(ptr) })
			}
		}
	}

	pub fn error(&self) -> Error {
		unsafe {
			Error::from(error::ALC(alcGetError(self.as_ptr())))
		}
	}
}

impl<'a> Drop for Device<'a> {
	fn drop(&mut self) {
		unsafe {
			if self._own {
				if alcCloseDevice(self.as_mut_ptr()) != ALC_TRUE {
					panic!("device still in use");
				}
			}
		}
	}
}
