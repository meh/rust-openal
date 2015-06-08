use std::marker::Reflect;
use std::any::TypeId;

use ffi::*;
use ::Error;

#[derive(PartialEq, Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Vector {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

#[derive(PartialEq, Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Position(pub Vector);

#[derive(PartialEq, Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Direction(pub Vector);

#[derive(PartialEq, Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Velocity(pub Vector);

#[derive(PartialEq, Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Orientation(pub Vector, pub Vector);

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Doppler {
	pub factor:   f32,
	pub velocity: f32,
}

pub fn format_for<T: Reflect + 'static>(channels: u16) -> Result<ALenum, Error> {
	if channels != 1 && channels != 2 {
		return Err(Error::InvalidValue);
	}

	if TypeId::of::<T>() == TypeId::of::<u8>() {
		if channels == 1 {
			Ok(AL_FORMAT_MONO8)
		}
		else {
			Ok(AL_FORMAT_STEREO8)
		}
	}
	else if TypeId::of::<T>() == TypeId::of::<i16>() {
		if channels == 1 {
			Ok(AL_FORMAT_MONO16)
		}
		else {
			Ok(AL_FORMAT_STEREO16)
		}
	}
	else {
		Err(Error::InvalidValue)
	}
}
