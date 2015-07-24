use ffi::*;
use Error;

/// A 3D vector.
#[derive(PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub struct Vector {
	/// The x.
	pub x: f32,

	/// The y.
	pub y: f32,

	/// The z.
	pub z: f32,
}

/// A 3D vector representing position.
#[derive(PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub struct Position(pub Vector);

/// A 3D vector representing direction.
#[derive(PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub struct Direction(pub Vector);

/// A 3D vector representing velocity.
#[derive(PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub struct Velocity(pub Vector);

/// Two 3D vectors representing orientation.
#[derive(PartialEq, Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Orientation(pub Vector, pub Vector);

/// The doppler.
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Doppler {
	/// The factor.
	pub factor: f32,

	/// The velocity.
	pub velocity: f32,
}

/// A trait for defining a conversion between a type and an OpenAL format.
pub trait Sample {
	/// Returns the OpenAL format based on the number of channels.
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

#[doc(hidden)]
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

#[doc(hidden)]
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
