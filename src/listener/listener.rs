use std::ffi::{CString, CStr};
use std::ptr;
use std::mem;
use std::str::from_utf8_unchecked;
use std::marker::PhantomData;

use ffi::*;
use {Error, Device, Context, Source, Sample, Buffer};
use super::Attributes;
use ::util::{Vector, Position, Velocity, Orientation, Doppler};

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
	pub fn default(attributes: &Attributes) -> Result<Self, Error> {
		unsafe {
			if !alcGetCurrentContext().is_null() {
				return Err(Error::InvalidOperation);
			}

			let device = alcOpenDevice(ptr::null());

			if device.is_null() {
				return Err(Error::InvalidName);
			}

			let context = alcCreateContext(device, Vec::from(attributes).as_ptr());

			if context.is_null() {
				al_try!(&device, ());
			}

			if alcMakeContextCurrent(context) != ALC_TRUE {
				al_try!(&device, ());
			}

			Ok(Listener::wrap(device, context))
		}
	}

	pub fn open(name: &str, attributes: &Attributes) -> Result<Self, Error> {
		unsafe {
			if !alcGetCurrentContext().is_null() {
				return Err(Error::InvalidOperation);
			}

			let device = alcOpenDevice(CString::new(name.as_bytes()).unwrap().as_ptr());

			if device.is_null() {
				return Err(Error::InvalidName);
			}

			let context = alcCreateContext(device, Vec::from(attributes).as_ptr());

			if context.is_null() {
				al_try!(&device, ());
			}

			if alcMakeContextCurrent(context) != ALC_TRUE {
				al_try!(&device, ());
			}

			Ok(Listener::wrap(device, context))
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

	pub fn source<'b>(&self) -> Result<Source<'b>, Error> where 'a: 'b {
		unsafe {
			Source::new()
		}
	}

	pub fn buffer<'b, T: Sample>(&self, channels: u16, data: &[T], rate: u32) -> Result<Buffer<'b>, Error> where 'a: 'b {
		unsafe {
			Buffer::new(channels, data, rate)
		}
	}

	pub fn vendor(&self) -> &'static str {
		unsafe {
			from_utf8_unchecked(CStr::from_ptr(alGetString(AL_VENDOR)).to_bytes())
		}
	}

	pub fn version(&self) -> (&'static str, &'static str) {
		unsafe {
			let     string = from_utf8_unchecked(CStr::from_ptr(alGetString(AL_VERSION)).to_bytes());
			let mut pieces = string.splitn(2, ' ');

			(pieces.next().unwrap(), pieces.next().unwrap())
		}
	}

	pub fn renderer(&self) -> &'static str {
		unsafe {
			from_utf8_unchecked(CStr::from_ptr(alGetString(AL_RENDERER)).to_bytes())
		}
	}

	pub fn extensions(&self) -> Vec<&'static str> {
		unsafe {
			from_utf8_unchecked(CStr::from_ptr(alGetString(AL_EXTENSIONS)).to_bytes())
				.split(' ')
				.collect()
		}
	}

	pub fn doppler(&self) -> Doppler {
		unsafe {
			Doppler {
				factor:   alGetFloat(AL_DOPPLER_FACTOR),
				velocity: alGetFloat(AL_DOPPLER_VELOCITY),
			}
		}
	}

	pub fn set_doppler(&mut self, value: Doppler) {
		unsafe {
			alDopplerFactor(value.factor);
			alDopplerVelocity(value.velocity);
		}
	}

	pub fn speed_of_sound(&self) -> f32 {
		unsafe {
			alGetFloat(AL_SPEED_OF_SOUND)
		}
	}

	pub fn set_speed_of_sound(&mut self, value: f32) {
		unsafe {
			alSpeedOfSound(value as ALfloat);
		}
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

unsafe impl<'a> Device for Listener<'a> {
	fn as_ptr(&self) -> *const ALCdevice {
		self.device as *const _
	}
}

unsafe impl<'a> Context for Listener<'a> {
	fn as_ptr(&self) -> *const ALCcontext {
		self.context as *const _
	}
}

impl<'a> Drop for Listener<'a> {
	fn drop(&mut self) {
		unsafe {
			if alcMakeContextCurrent(ptr::null_mut()) != ALC_TRUE {
				al_panic!(self);
			}

			alcDestroyContext(self.context);
			al_panic!(self);

			if alcCloseDevice(self.device) != ALC_TRUE {
				al_panic!(self);
			}
		}
	}
}
