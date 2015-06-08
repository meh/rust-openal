use std::marker::{Reflect, PhantomData};
use std::ptr;
use std::ffi::CString;

use ffi::*;
use ::{Error, traits};
use ::util::format_for;
use ::device::extension;

pub struct Capture<T: Reflect + 'static> {
	ptr: *mut ALCdevice,

	_marker: PhantomData<T>,
}

impl<T: Reflect + 'static> Capture<T> {
	pub unsafe fn wrap(ptr: *mut ALCdevice) -> Self {
		Capture { ptr: ptr, _marker: PhantomData }
	}

	pub unsafe fn as_ptr(&self) -> *const ALCdevice {
		self.ptr as *const _
	}

	pub unsafe fn as_mut_ptr(&mut self) -> *mut ALCdevice {
		self.ptr
	}
}

impl<T: Reflect + 'static> Capture<T> {
	pub fn default<U: Reflect + 'static>(channels: u16, rate: u32, size: usize) -> Result<Capture<U>, Error> {
		unsafe {
			let ptr = alcCaptureOpenDevice(ptr::null(),
				rate as ALCuint,
				try!(format_for::<U>(channels)),
				size as ALCsizei);

			if ptr.is_null() {
				Err(Error::InvalidName)
			}
			else {
				Ok(Capture::wrap(ptr))
			}
		}
	}

	pub fn open<U: Reflect + 'static>(name: &str, channels: u16, rate: u32, size: usize) -> Result<Capture<U>, Error> {
		unsafe {
			let ptr = alcCaptureOpenDevice(CString::new(name.as_bytes()).unwrap().as_ptr(),
				rate as ALCuint,
				try!(format_for::<U>(channels)),
				size as ALCsizei);

			if ptr.is_null() {
				Err(Error::InvalidName)
			}
			else {
				Ok(Capture::wrap(ptr))
			}
		}
	}

	pub fn start(&mut self) {
		unsafe {
			alcCaptureStart(self.as_mut_ptr());
		}
	}

	pub fn stop(&mut self) {
		unsafe {
			alcCaptureStop(self.as_mut_ptr());
		}
	}

	pub fn samples(&self) -> usize {
		unsafe {
			let mut value = 0;
			alcGetIntegerv(self.as_ptr(), ALC_CAPTURE_SAMPLES, 1, &mut value);

			value as usize
		}
	}

	pub fn take(&mut self, out: &mut [T]) -> Result<(), Error> {
		unsafe {
			alcCaptureSamples(self.as_mut_ptr(), out.as_mut_ptr() as *mut _, out.len() as ALCsizei);

			if let Some(error) = Error::last_for(self) {
				Err(error)
			}
			else {
				Ok(())
			}
		}
	}
}

impl<T: Reflect + 'static> Drop for Capture<T> {
	fn drop(&mut self) {
		unsafe {
			if cfg!(debug_assertions) {
				if alcCaptureCloseDevice(self.as_mut_ptr()) != ALC_TRUE {
					panic!("{}", Error::last_for(self).unwrap());
				}
			}
			else {
				alcCaptureCloseDevice(self.as_mut_ptr());
			}
		}
	}
}

unsafe impl<T: Reflect + 'static> traits::Device for Capture<T> {
	fn as_ptr(&self) -> *const ALCdevice {
		self.ptr as *const _
	}
}

pub fn default<T: Reflect + 'static>(channels: u16, rate: u32, size: usize) -> Result<Capture<T>, Error> {
	Capture::<T>::default(channels, rate, size)
}

pub fn names() -> Vec<&'static str> {
	use std::ffi::CStr;
	use std::str::from_utf8_unchecked;
	use libc::strlen;

	let mut result = Vec::new();

	unsafe {
		if extension::is_supported("ALC_ENUMERATION_EXT") {
			let mut ptr = alcGetString(ptr::null(), ALC_CAPTURE_DEVICE_SPECIFIER);

			while *ptr != 0 {
				result.push(from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()));

				ptr = ptr.offset(strlen(ptr) as isize + 1);
			}
		}
	}

	result
}
