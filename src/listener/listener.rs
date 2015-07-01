use std::ffi::{CString, CStr};
use std::ptr;
use std::mem;
use std::str::from_utf8_unchecked;
use std::marker::PhantomData;

use ffi::*;
use {Error, Device, Context, Source, Sample, Buffer};
use super::Attributes;
use ::util::{Vector, Position, Velocity, Orientation, Doppler};

/// Represents the listener.
///
/// Only one `Listener` may be open at a time.
///
/// _In OpenAL parlance a `Listener` contains both an OpenAL device and an
/// OpenAL context, and that context is made current on creation. This is
/// because there are some inconsistencies between implementations that make
/// having multiple devices or contexts impossible._
pub struct Listener<'a> {
	device:  *mut ALCdevice,
	context: *mut ALCcontext,

	_marker: PhantomData<&'a ()>,
}

unsafe impl<'a> Send for Listener<'a> { }

impl<'a> Listener<'a> {
	#[doc(hidden)]
	pub unsafe fn wrap(device: *mut ALCdevice, context: *mut ALCcontext) -> Self {
		Listener { device: device, context: context, _marker: PhantomData }
	}
}

impl<'a> Listener<'a> {
	#[doc(hidden)]
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

	#[doc(hidden)]
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

	/// Process the `Listener`. See OpenAL documentation for `alcProcessContext`.
	pub fn process(&mut self) {
		unsafe {
			alcProcessContext(self.context);
		}
	}

	/// Suspend the `Listener`. See OpenAL documentation for `alcSuspendContext`.
	pub fn suspend(&mut self) {
		unsafe {
			alcSuspendContext(self.context);
		}
	}

	/// Create a new `Source`.
	pub fn source<'b>(&self) -> Result<Source<'b>, Error> where 'a: 'b {
		unsafe {
			Source::new()
		}
	}

	/// Create a new `Buffer` and fill it.
	pub fn buffer<'b, T: Sample>(&self, channels: u16, data: &[T], rate: u32) -> Result<Buffer<'b>, Error> where 'a: 'b {
		unsafe {
			Buffer::new(channels, data, rate)
		}
	}

	/// Get the vendor name.
	pub fn vendor(&self) -> &'static str {
		unsafe {
			from_utf8_unchecked(CStr::from_ptr(alGetString(AL_VENDOR)).to_bytes())
		}
	}

	/// Get the OpenAL specification version and the context specific version.
	pub fn version(&self) -> (&'static str, &'static str) {
		unsafe {
			let     string = from_utf8_unchecked(CStr::from_ptr(alGetString(AL_VERSION)).to_bytes());
			let mut pieces = string.splitn(2, ' ');

			(pieces.next().unwrap(), pieces.next().unwrap())
		}
	}

	/// Get the name of the renderer.
	pub fn renderer(&self) -> &'static str {
		unsafe {
			from_utf8_unchecked(CStr::from_ptr(alGetString(AL_RENDERER)).to_bytes())
		}
	}

	/// Get a list of extensions supported.
	pub fn extensions(&self) -> Vec<&'static str> {
		unsafe {
			from_utf8_unchecked(CStr::from_ptr(alGetString(AL_EXTENSIONS)).to_bytes())
				.split(' ')
				.collect()
		}
	}

	/// Get the doppler factor and velocity.
	pub fn doppler(&self) -> Doppler {
		unsafe {
			Doppler {
				factor:   alGetFloat(AL_DOPPLER_FACTOR),
				velocity: alGetFloat(AL_DOPPLER_VELOCITY),
			}
		}
	}

	/// Set the doppler factor and velocity.
	pub fn set_doppler(&mut self, value: Doppler) {
		unsafe {
			alDopplerFactor(value.factor);
			alDopplerVelocity(value.velocity);
		}
	}

	/// Get the speed of sound.
	pub fn speed_of_sound(&self) -> f32 {
		unsafe {
			alGetFloat(AL_SPEED_OF_SOUND)
		}
	}

	/// Set the speed of sound.
	pub fn set_speed_of_sound(&mut self, value: f32) {
		unsafe {
			alSpeedOfSound(value as ALfloat);
		}
	}

	/// Get the listener gain.
	pub fn gain(&self) -> f32 {
		unsafe {
			let mut value = 0.0;
			alGetListenerf(AL_GAIN, &mut value);

			value as f32
		}
	}

	/// Set the listener gain.
	pub fn set_gain(&mut self, value: f32) {
		unsafe {
			alListenerf(AL_GAIN, value as ALfloat);
		}
	}

	/// Get the listener position.
	pub fn position(&self) -> Position {
		unsafe {
			let mut value = Position(Vector { x: 0.0, y: 0.0, z: 0.0 });
			alGetListenerfv(AL_POSITION, mem::transmute(&mut value));

			value
		}
	}

	/// Set the listener position.
	pub fn set_position(&mut self, value: &Position) {
		unsafe {
			alListenerfv(AL_POSITION, mem::transmute(value));
		}
	}

	/// Get the listener velocity.
	pub fn velocity(&self) -> Velocity {
		unsafe {
			let mut value = Velocity(Vector { x: 0.0, y: 0.0, z: 0.0 });
			alGetListenerfv(AL_VELOCITY, mem::transmute(&mut value));

			value
		}
	}

	/// Set the listener velocity.
	pub fn set_velocity(&mut self, value: &Velocity) {
		unsafe {
			alListenerfv(AL_VELOCITY, mem::transmute(value));
		}
	}

	/// Get the listener orientation.
	pub fn orientation(&self) -> Orientation {
		unsafe {
			let mut value = Orientation(Vector { x: 0.0, y: 0.0, z: 0.0 }, Vector { x: 0.0, y: 0.0, z: 0.0 });
			alGetListenerfv(AL_ORIENTATION, mem::transmute(&mut value));

			value
		}
	}

	/// Set the listener orientation.
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

impl<'a> ::std::fmt::Debug for Listener<'a> {
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
		try!(f.write_str("openal::Listener("));

		try!(f.write_str(&format!("vendor={:?} ", self.vendor())));
		try!(f.write_str(&format!("version={:?} ", self.version())));
		try!(f.write_str(&format!("renderer={:?} ", self.renderer())));
		try!(f.write_str(&format!("extensions={:?}; ", self.extensions())));

		try!(f.write_str(&format!("doppler={:?} ", self.doppler())));
		try!(f.write_str(&format!("speed_of_sound={} ", self.speed_of_sound())));
		try!(f.write_str(&format!("gain={} ", self.gain())));
		try!(f.write_str(&format!("position={:?} ", self.position())));
		try!(f.write_str(&format!("velocity={:?} ", self.velocity())));
		try!(f.write_str(&format!("orientation={:?}", self.orientation())));

		f.write_str(")")
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
