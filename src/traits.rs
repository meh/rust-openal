use ffi::*;

pub unsafe trait Device {
	fn as_ptr(&self) -> *const ALCdevice;

	fn as_mut_ptr(&mut self) -> *mut ALCdevice {
		self.as_ptr() as *mut _
	}
}

unsafe impl Device for *const ALCdevice {
	fn as_ptr(&self) -> *const ALCdevice {
		*self
	}
}

unsafe impl Device for *mut ALCdevice {
	fn as_ptr(&self) -> *const ALCdevice {
		*self
	}
}
