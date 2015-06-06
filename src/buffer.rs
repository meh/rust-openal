use std::mem;
use std::marker::Reflect;
use std::any::TypeId;

use ffi::*;
use ::Error;

#[derive(PartialEq, Eq)]
pub struct Buffer {
	id: ALuint,
}

impl Buffer {
	pub unsafe fn id(&self) -> ALuint {
		self.id
	}
}

impl Buffer {
	pub fn empty() -> Self {
		unsafe {
			let mut id = 0;
			alGenBuffers(1, &mut id);

			Buffer { id: id }
		}
	}

	pub fn new<T: Reflect + 'static>(channels: u16, data: &[T], rate: u32) -> Result<Self, Error> {
		let mut buffer = Buffer::empty();

		match buffer.fill(channels, data, rate) {
			Ok(..) =>
				Ok(buffer),

			Err(error) =>
				Err(error)
		}
	}

	pub fn fill<T: Reflect + 'static>(&mut self, channels: u16, data: &[T], rate: u32) -> Result<(), Error> {
		unsafe {
			alBufferData(self.id, try!(format_for::<T>(channels)), data.as_ptr() as *const _,
				(mem::size_of::<T>() * data.len()) as ALsizei, rate as ALint);

			if let Some(error) = Error::last() {
				Err(error)
			}
			else {
				Ok(())
			}
		}
	}

	pub fn rate(&self) -> u32 {
		unsafe {
			let mut value = 0;
			alGetBufferi(self.id, AL_FREQUENCY, &mut value);

			value as u32
		}
	}

	pub fn bits(&self) -> u16 {
		unsafe {
			let mut value = 0;
			alGetBufferi(self.id, AL_BITS, &mut value);

			value as u16
		}
	}

	pub fn channels(&self) -> u16 {
		unsafe {
			let mut value = 0;
			alGetBufferi(self.id, AL_CHANNELS, &mut value);

			value as u16
		}
	}

	pub fn len(&self) -> usize {
		unsafe {
			let mut value = 0;
			alGetBufferi(self.id, AL_SIZE, &mut value);

			value as usize
		}
	}
}

impl Drop for Buffer {
	fn drop(&mut self) {
		unsafe {
			alDeleteBuffers(1, &self.id);

			if let Some(error) = Error::last() {
				panic!("{}", error)
			}
		}
	}
}

fn format_for<T: Reflect + 'static>(channels: u16) -> Result<ALenum, Error> {
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
