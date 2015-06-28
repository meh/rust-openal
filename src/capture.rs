use std::marker::PhantomData;
use std::ptr;
use std::ffi::CString;

use ffi::*;
use ::{Error, Sample, extension};
use traits::Device;

pub struct Capture<T: Sample> {
	ptr: *mut ALCdevice,

	_marker: PhantomData<T>,
}

impl<T: Sample> Capture<T> {
	pub unsafe fn wrap(ptr: *mut ALCdevice) -> Self {
		Capture { ptr: ptr, _marker: PhantomData }
	}
}

impl<T: Sample> Capture<T> {
	pub fn default<U: Sample>(channels: u16, rate: u32, size: usize) -> Result<Capture<U>, Error> {
		unsafe {
			let ptr = alcCaptureOpenDevice(ptr::null(),
				rate as ALCuint,
				try!(<U as Sample>::format(channels)),
				size as ALCsizei);

			if ptr.is_null() {
				Err(Error::InvalidName)
			}
			else {
				Ok(Capture::wrap(ptr))
			}
		}
	}

	pub fn open<U: Sample>(name: &str, channels: u16, rate: u32, size: usize) -> Result<Capture<U>, Error> {
		unsafe {
			let ptr = alcCaptureOpenDevice(CString::new(name.as_bytes()).unwrap().as_ptr(),
				rate as ALCuint,
				try!(<U as Sample>::format(channels)),
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

impl<T: Sample> Drop for Capture<T> {
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

unsafe impl<T: Sample> Device for Capture<T> {
	fn as_ptr(&self) -> *const ALCdevice {
		self.ptr as *const _
	}
}

pub fn default<T: Sample>(channels: u16, rate: u32, size: usize) -> Result<Capture<T>, Error> {
	Capture::<T>::default(channels, rate, size)
}

pub fn open<T: Sample>(name: &str, channels: u16, rate: u32, size: usize) -> Result<Capture<T>, Error> {
	Capture::<T>::open(name, channels, rate, size)
}

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
