use ffi::*;
use ::{Error, Source};

#[must_use]
pub struct UnsafeBuffer {
	source: ALuint,
	buffer: ALuint,
}

impl UnsafeBuffer {
	pub unsafe fn new(source: &Source, buffer: &::Buffer) -> Result<Self, Error> {
		alSourceQueueBuffers(source.id(), 1, &buffer.id());

		if let Some(error) = Error::last() {
			Err(error)
		}
		else {
			Ok(UnsafeBuffer { source: source.id(), buffer: buffer.id() })
		}
	}
}

impl Drop for UnsafeBuffer {
	fn drop(&mut self) {
		unsafe {
			alSourceUnqueueBuffers(self.source, 1, &self.buffer);

			if cfg!(debug_assertions) {
				if let Some(error) = Error::last() {
					panic!("{}", error);
				}
			}
		}
	}
}
