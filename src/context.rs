use std::ptr;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use ffi::*;
use ::{Device, Error};
use ::{Vector, Position, Velocity, Orientation};

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
				Err(Error::last_for(device).unwrap())
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

	pub fn make_current(mut self) -> Result<Current<'a>, Error> {
		unsafe {
			if cfg!(debug_assertions) {
				if !alcGetCurrentContext().is_null() {
					panic!("there's already a current context");
				}
			}

			if alcMakeContextCurrent(self.as_mut_ptr()) == ALC_TRUE {
				Ok(Current::wrap(self))
			}
			else {
				Err(Error::last_for(&self.device()).unwrap())
			}
		}
	}

	pub unsafe fn just_make_current(&mut self) -> Result<(), Error> {
		if alcMakeContextCurrent(self.as_mut_ptr()) == ALC_TRUE {
			Ok(())
		}
		else {
			Err(Error::last_for(&self.device()).unwrap())
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

			if cfg!(debug_assertions) {
				if let Some(error) = Error::last() {
					panic!("{}", error);
				}
			}
		}
	}
}

#[must_use]
pub struct Current<'a>(Option<Context<'a>>);

impl<'a> Current<'a> {
	pub unsafe fn wrap(context: Context) -> Current {
		Current(Some(context))
	}
}

impl<'a> Current<'a> {
	pub fn take(mut self) -> Context<'a> {
		unsafe {
			alcMakeContextCurrent(ptr::null_mut());
		}

		self.0.take().unwrap()
	}

	pub fn gain(&self) -> f32 {
		unsafe {
			let mut value = 0.0;
			alGetListenerf(AL_GAIN, &mut value);

			value as f32
		}
	}

	pub fn set_gain(&mut self, value: f32) {
		unsafe {
			alListenerf(AL_GAIN, value as ALfloat);
		}
	}

	pub fn position(&self) -> Position {
		unsafe {
			let mut value = Position(Vector { x: 0.0, y: 0.0, z: 0.0 });
			alGetListenerfv(AL_POSITION, mem::transmute(&mut value));

			value
		}
	}

	pub fn set_position(&mut self, value: &Position) {
		unsafe {
			alListenerfv(AL_POSITION, mem::transmute(value));
		}
	}

	pub fn velocity(&self) -> Velocity {
		unsafe {
			let mut value = Velocity(Vector { x: 0.0, y: 0.0, z: 0.0 });
			alGetListenerfv(AL_VELOCITY, mem::transmute(&mut value));

			value
		}
	}

	pub fn set_velocity(&mut self, value: &Velocity) {
		unsafe {
			alListenerfv(AL_VELOCITY, mem::transmute(value));
		}
	}

	pub fn orientation(&self) -> Orientation {
		unsafe {
			let mut value = Orientation(Vector { x: 0.0, y: 0.0, z: 0.0 }, Vector { x: 0.0, y: 0.0, z: 0.0 });
			alGetListenerfv(AL_ORIENTATION, mem::transmute(&mut value));

			value
		}
	}

	pub fn set_orientation(&mut self, value: &Orientation) {
		unsafe {
			alListenerfv(AL_ORIENTATION, mem::transmute(value));
		}
	}
}

impl<'a> Deref for Current<'a> {
	type Target = Context<'a>;

	fn deref(&self) -> &<Self as Deref>::Target {
		self.0.as_ref().unwrap()
	}
}

impl<'a> DerefMut for Current<'a> {
	fn deref_mut(&mut self) -> &mut<Self as Deref>::Target {
		self.0.as_mut().unwrap()
	}
}

impl<'a> Drop for Current<'a> {
	fn drop(&mut self) {
		unsafe {
			if self.0.is_some() {
				alcMakeContextCurrent(ptr::null_mut());
			}
		}
	}
}
