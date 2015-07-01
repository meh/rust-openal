use ffi::*;
use Error;

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

pub trait Sample {
	fn format(channels: u16) -> Result<ALenum, Error>;
}

impl Sample for u8 {
	fn format(channels: u16) -> Result<ALenum, Error> {
		match channels {
			1 => Ok(AL_FORMAT_MONO8),
			2 => Ok(AL_FORMAT_STEREO8),
			_ => Err(Error::InvalidValue),
		}
	}
}

impl Sample for i16 {
	fn format(channels: u16) -> Result<ALenum, Error> {
		match channels {
			1 => Ok(AL_FORMAT_MONO16),
			2 => Ok(AL_FORMAT_STEREO16),
			_ => Err(Error::InvalidValue),
		}
	}
}

macro_rules! al_panic {
	() => (
		if cfg!(debug_assertions) {
			if let Some(error) = ::Error::last() {
				panic!("{}", error);
			}
		}
	);

	($device:expr) => (
		if cfg!(debug_assertions) {
			if let Some(error) = ::Error::last_for($device) {
				panic!("{}", error);
			}
		}
	);
}

macro_rules! al_try {
	($body:expr) => ({
		let result = { $body };

		if let Some(error) = ::Error::last() {
			return Err(error);
		}

		result
	});

	($device:expr, $body:expr) => ({
		let result = { $body };

		if let Some(error) = ::Error::last_for($device) {
			return Err(error);
		}

		result
	});
}
