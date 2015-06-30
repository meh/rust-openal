use std::marker::PhantomData;
use std::ptr;
use std::ffi::CString;

use ffi::*;
use ::{Error, Sample};
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
			al_try!(self,
				alcCaptureSamples(self.as_mut_ptr(), out.as_mut_ptr() as *mut _, out.len() as ALCsizei));

			Ok(())
		}
	}
}

impl<T: Sample> Drop for Capture<T> {
	fn drop(&mut self) {
		unsafe {
			alcCaptureCloseDevice(self.as_mut_ptr());
			al_panic!(self);
		}
	}
}

unsafe impl<T: Sample> Device for Capture<T> {
	fn as_ptr(&self) -> *const ALCdevice {
		self.ptr as *const _
	}
}
