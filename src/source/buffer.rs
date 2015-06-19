use std::ops::{Deref, DerefMut};

use ffi::*;
use ::{Error, Source};

#[must_use]
pub struct Buffer {
	source: ALuint,
	buffer: Option<::Buffer>,
}

impl Buffer {
	pub unsafe fn new(source: &Source, buffer: ::Buffer) -> Result<Buffer, Error> {
		alSourceQueueBuffers(source.id(), 1, &buffer.id());

		if let Some(error) = Error::last() {
			Err(error)
		}
		else {
			Ok(Buffer { source: source.id(), buffer: Some(buffer) })
		}
	}

	pub fn take(mut self) -> Result<::Buffer, Error> {
		unsafe {
			alSourceUnqueueBuffers(self.source, 1, &self.buffer.as_ref().unwrap().id());

			if let Some(error) = Error::last() {
				Err(error)
			}
			else {
				Ok(self.buffer.take().unwrap())
			}
		}
	}
}

impl Drop for Buffer {
	fn drop(&mut self) {
		unsafe {
			if let Some(ref buffer) = self.buffer {
				alSourceUnqueueBuffers(self.source, 1, &buffer.id());

				if cfg!(debug_assertions) {
					if let Some(error) = Error::last() {
						panic!("{}", error);
					}
				}
			}
		}
	}
}

impl Deref for Buffer {
	type Target = ::Buffer;

	fn deref(&self) -> &<Self as Deref>::Target {
		self.buffer.as_ref().unwrap()
	}
}

impl DerefMut for Buffer {
	fn deref_mut(&mut self) -> &mut<Self as Deref>::Target {
		self.buffer.as_mut().unwrap()
	}
}
