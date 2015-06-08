#![feature(core)]

extern crate libc;
extern crate openal_sys as ffi;

pub mod util;
pub use util::{Vector, Position, Direction, Velocity, Orientation};

pub mod error;
pub use error::Error;

pub mod device;
pub use device::Device;

pub mod context;
pub use context::Context;

pub mod source;
pub use source::Source;

pub mod buffer;
pub use buffer::Buffer;

pub mod extension;

pub mod listener;

use std::ffi::CStr;
use std::str::from_utf8_unchecked;

use ffi::*;

pub fn vendor() -> Option<&'static str> {
	unsafe {
		let ptr = alGetString(AL_VENDOR);

		if ptr.is_null() {
			None
		}
		else {
			Some(from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()))
		}
	}
}

pub fn version() -> Option<&'static str> {
	unsafe {
		let ptr = alGetString(AL_VERSION);

		if ptr.is_null() {
			None
		}
		else {
			Some(from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()))
		}
	}
}

pub fn renderer() -> Option<&'static str> {
	unsafe {
		let ptr = alGetString(AL_RENDERER);

		if ptr.is_null() {
			None
		}
		else {
			Some(from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()))
		}
	}
}

pub fn extensions() -> Option<&'static str> {
	unsafe {
		let ptr = alGetString(AL_EXTENSIONS);

		if ptr.is_null() {
			None
		}
		else {
			Some(from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()))
		}
	}
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Doppler {
	factor:   f32,
	velocity: f32,
}

pub fn doppler() -> Doppler {
	unsafe {
		Doppler {
			factor:   alGetFloat(AL_DOPPLER_FACTOR),
			velocity: alGetFloat(AL_DOPPLER_VELOCITY),
		}
	}
}

pub fn set_doppler(value: Doppler) {
	unsafe {
		alDopplerFactor(value.factor);
		alDopplerVelocity(value.velocity);
	}
}

pub fn speed_of_sound() -> f32 {
	unsafe {
		alGetFloat(AL_SPEED_OF_SOUND)
	}
}

pub fn set_speed_of_sound(value: f32) {
	unsafe {
		alSpeedOfSound(value as ALfloat);
	}
}
