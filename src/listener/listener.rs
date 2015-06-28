use std::ffi::CString;
use std::ptr;
use std::marker::PhantomData;

use ffi::*;
use {Error, Device};

use super::Current;

pub struct Listener<'a> {
	device:  *mut ALCdevice,
	context: *mut ALCcontext,

	_marker: PhantomData<&'a ()>,
}

unsafe impl<'a> Send for Listener<'a> { }

impl<'a> Listener<'a> {
	pub unsafe fn wrap(device: *mut ALCdevice, context: *mut ALCcontext) -> Self {
		Listener { device: device, context: context, _marker: PhantomData }
	}
}

impl<'a> Listener<'a> {
	pub fn default() -> Result<Self, Error> {
		unsafe {
			let device = alcOpenDevice(ptr::null());

			if device.is_null() {
				return Err(Error::InvalidName);
			}

			let context = alcCreateContext(device, vec!(0).as_ptr());

			if context.is_null() {
				return Err(Error::last_for(&device).unwrap());
			}

			Ok(Listener::wrap(device, context))
		}
	}

	pub fn open(name: &str) -> Result<Self, Error> {
		unsafe {
			let device = alcOpenDevice(CString::new(name.as_bytes()).unwrap().as_ptr());

			if device.is_null() {
				return Err(Error::InvalidName);
			}

			let context = alcCreateContext(device, vec!(0).as_ptr());

			if context.is_null() {
				return Err(Error::last_for(&device).unwrap());
			}

			Ok(Listener::wrap(device, context))
		}
	}

	pub fn make_current<'b>(&'b mut self) -> Result<Current<'b, 'a>, Error> {
		unsafe {
			if !alcGetCurrentContext().is_null() {
				Err(Error::InvalidOperation)
			}
			else {
				alcMakeContextCurrent(self.context);

				Ok(Current::wrap(self))
			}
		}
	}

	pub fn is_current(&self) -> bool {
		unsafe {
			alcGetCurrentContext() == self.context
		}
	}

	pub fn process(&mut self) {
		unsafe {
			alcProcessContext(self.context);
		}
	}

	pub fn suspend(&mut self) {
		unsafe {
			alcSuspendContext(self.context);
		}
	}
}

unsafe impl<'a> Device for Listener<'a> {
	fn as_ptr(&self) -> *const ALCdevice {
		self.device as *const _
	}
}

impl<'a> Drop for Listener<'a> {
	fn drop(&mut self) {
		unsafe {
			if cfg!(debug_assertions) {
				alcDestroyContext(self.context);

				if let Some(error) = Error::last() {
					panic!("{}", error);
				}

				if alcCloseDevice(self.device) != ALC_TRUE {
					panic!("{}", Error::last_for(self).unwrap());
				}
			}
			else {
				alcDestroyContext(self.context);
				alcCloseDevice(self.device);
			}
		}
	}
}
