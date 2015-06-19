use std::ops::{Deref, DerefMut};
use std::collections::VecDeque;

use ::{Error, Source, Sample};
use super::Buffer;

pub struct Buffered {
	source:  Source,
	buffers: VecDeque<Buffer>,
}

impl Buffered {
	pub fn new(source: Source) -> Buffered {
		Buffered {
			source:  source,
			buffers: VecDeque::new(),
		}
	}

	pub fn push<T: Sample>(&mut self, channels: u16, data: &[T], rate: u32) -> Result<(), Error> {
		let buffer = try!(::Buffer::new(channels, data, rate));
		let buffer = try!(unsafe { Buffer::new(self, buffer) });

		self.clear();
		self.buffers.push_back(buffer);

		Ok(())
	}

	pub fn pop(&mut self) -> Result<::Buffer, Error> {
		if let Some(buffer) = self.buffers.pop_back() {
			buffer.take()
		}
		else {
			Err(Error::InvalidOperation)
		}
	}

	pub fn clear(&mut self) {
		for _ in 0 .. self.processed() {
			self.buffers.pop_front();
		}
	}

	pub fn queue(&mut self) -> ! {
		unreachable!();
	}

	pub unsafe fn just_queue(&mut self) -> ! {
		unreachable!();
	}
}

impl Deref for Buffered {
	type Target = Source;

	fn deref(&self) -> &<Self as Deref>::Target {
		&self.source
	}
}

impl DerefMut for Buffered {
	fn deref_mut(&mut self) -> &mut<Self as Deref>::Target {
		&mut self.source
	}
}

impl Drop for Buffered {
	fn drop(&mut self) {
		self.buffers.clear();
	}
}
