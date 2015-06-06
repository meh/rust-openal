use std::marker::PhantomData;

use ffi::*;
use ::{Device, Error};

pub struct Context<'a> {
	ptr: *mut ALCcontext,

	_marker: PhantomData<&'a ()>,
}

impl<'a> Context<'a> {
	pub unsafe fn wrap<'b>(ptr: *mut ALCcontext) -> Context<'b> {
		Context { ptr: ptr, _marker: PhantomData }
	}

	pub unsafe fn as_ptr(&self) -> *const ALCcontext {
		self.ptr as *const _
	}

	pub unsafe fn as_mut_ptr(&mut self) -> *mut ALCcontext {
		self.ptr
	}
}

impl<'a> Context<'a> {
	pub fn new(device: &Device) -> Result<Self, Error> {
		Context::new_with(device, &[])
	}

	pub fn new_with(device: &Device, attributes: &[ALCint]) -> Result<Self, Error> {
		unsafe {
			let mut terminated = attributes.to_vec();
			terminated.push(0);

			let ptr = alcCreateContext(device.as_ptr(), terminated.as_ptr());

			if ptr.is_null() {
				Err(device.error())
			}
			else {
				Ok(Context::wrap(ptr))
			}
		}
	}

	pub fn device(&self) -> Device {
		unsafe {
			Device::wrap(alcGetContextsDevice(self.as_ptr()))
		}
	}

	pub fn process(&mut self) {
		unsafe {
			alcProcessContext(self.as_mut_ptr());
		}
	}

	pub fn suspend(&mut self) {
		unsafe {
			alcSuspendContext(self.as_mut_ptr());
		}
	}

	pub fn make_current(&mut self) -> Result<(), Error> {
		unsafe {
			if alcMakeContextCurrent(self.as_mut_ptr()) == ALC_TRUE {
				Ok(())
			}
			else {
				Err(self.device().error())
			}
		}
	}

	pub fn is_current(&self) -> bool {
		unsafe {
			alcGetCurrentContext() == self.as_ptr()
		}
	}
}

impl<'a> Drop for Context<'a> {
	fn drop(&mut self) {
		unsafe {
			alcDestroyContext(self.as_mut_ptr());
		}
	}
}
