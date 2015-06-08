use std::error;
use std::fmt;

use ffi::*;
use super::Device;

#[derive(Copy, Clone)]
pub enum Error {
	None,

	InvalidDevice,
	InvalidContext,

	InvalidName,
	InvalidOperation,

	InvalidEnum,
	InvalidValue,

	OutOfMemory,
}

#[derive(Copy, Clone)]
pub struct AL(pub ALenum);

#[derive(Copy, Clone)]
pub struct ALC(pub ALCenum);

impl Error {
	pub fn last() -> Option<Self> {
		unsafe {
			match Error::from(AL(alGetError())) {
				Error::None =>
					None,

				error =>
					Some(error)
			}
		}
	}

	pub fn last_for(device: &Device) -> Option<Self> {
		unsafe {
			match Error::from(ALC(alcGetError(device.as_ptr()))) {
				Error::None =>
					None,

				error =>
					Some(error)
			}
		}
	}
}

impl From<AL> for Error {
	fn from(value: AL) -> Error {
		match value.0 {
			AL_NO_ERROR          => Error::None,
			AL_INVALID_NAME      => Error::InvalidName,
			AL_INVALID_ENUM      => Error::InvalidEnum,
			AL_INVALID_VALUE     => Error::InvalidValue,
			AL_INVALID_OPERATION => Error::InvalidOperation,
			AL_OUT_OF_MEMORY     => Error::OutOfMemory,

			_ => Error::None,
		}
	}
}

impl From<ALC> for Error {
	fn from(value: ALC) -> Error {
		match value.0 {
			ALC_NO_ERROR        => Error::None,
			ALC_INVALID_CONTEXT => Error::InvalidContext,
			ALC_INVALID_DEVICE  => Error::InvalidDevice,
			ALC_INVALID_ENUM    => Error::InvalidEnum,
			ALC_INVALID_VALUE   => Error::InvalidValue,
			ALC_OUT_OF_MEMORY   => Error::OutOfMemory,

			_ => Error::None,
		}
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		f.write_str(error::Error::description(self))
	}
}

impl fmt::Debug for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		try!(f.write_str("openal::Error("));
		try!(fmt::Display::fmt(self, f));
		f.write_str(")")
	}
}

impl error::Error for Error {
	fn description(&self) -> &str {
		match self {
			&Error::None =>
				"no error",

			&Error::InvalidDevice =>
				"invalid device",

			&Error::InvalidContext =>
				"invalid context",

			&Error::InvalidEnum =>
				"invalid enum",

			&Error::InvalidName =>
				"invalid name",

			&Error::InvalidValue =>
				"invalid value",

			&Error::InvalidOperation =>
				"invalid operation",

			&Error::OutOfMemory =>
				"out of memory",
		}
	}
}
