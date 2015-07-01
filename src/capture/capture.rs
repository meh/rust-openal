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

unsafe impl<T: Sample> Send for Capture<T> { }

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

	pub fn len(&self) -> usize {
		unsafe {
			let mut value = 0;
			alcGetIntegerv(self.as_ptr(), ALC_CAPTURE_SAMPLES, 1, &mut value);

			value as usize
		}
	}

	pub fn take(&mut self) -> Result<Vec<T>, Error> {
		unsafe {
			let mut result = Vec::with_capacity(self.len());

			al_try!(self,
				alcCaptureSamples(self.as_mut_ptr(), result.as_mut_ptr() as *mut _, self.len() as ALCsizei));

			Ok(result)
		}
	}
}

unsafe impl<T: Sample> Device for Capture<T> {
	fn as_ptr(&self) -> *const ALCdevice {
		self.ptr as *const _
	}
}

impl<T: Sample> ::std::fmt::Debug for Capture<T> {
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
		try!(f.write_str("openal::Capture("));
		try!(f.write_str(&format!("len={}; ", self.len())));
		f.write_str(")")
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
