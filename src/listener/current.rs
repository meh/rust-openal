use std::ffi::CStr;
use std::ptr;
use std::mem;
use std::str::from_utf8_unchecked;

use ffi::*;
use Source;
use ::util::{Vector, Position, Velocity, Orientation, Doppler};

use super::Listener;

#[must_use]
pub struct Current<'a, 'b: 'a>(&'a mut Listener<'b>);

impl<'a, 'b> Current<'a, 'b> {
	pub unsafe fn wrap(listener: &'a mut Listener<'b>) -> Self {
		Current(listener)
	}
}

impl<'a, 'b> Current<'a, 'b> {
	pub fn source(&self) -> Source<'b> {
		unsafe {
			Source::new()
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

impl<'a, 'b> Drop for Current<'a, 'b> {
	fn drop(&mut self) {
		unsafe {
			alcMakeContextCurrent(ptr::null_mut());
		}
	}
}
