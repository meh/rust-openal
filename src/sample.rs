use ffi::*;
use ::Error;

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
