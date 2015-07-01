use ffi::*;

/// Internal trait specifying the type is an OpenAL device.
pub unsafe trait Device {
	/// Return a `*const` pointer for the device.
	fn as_ptr(&self) -> *const ALCdevice;

	/// Return a `*mut` pointer for the device.
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

/// Internal trait specifying the type is an OpenAL context.
pub unsafe trait Context {
	/// Return a `*const` pointer for the context.
	fn as_ptr(&self) -> *const ALCcontext;

	/// Return a `*mut` pointer for the context.
	fn as_mut_ptr(&mut self) -> *mut ALCcontext {
		self.as_ptr() as *mut _
	}
}

unsafe impl Context for *const ALCcontext {
	fn as_ptr(&self) -> *const ALCcontext {
		*self
	}
}

unsafe impl Context for *mut ALCcontext {
	fn as_ptr(&self) -> *const ALCcontext {
		*self
	}
}
