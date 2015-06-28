use std::ops::{Deref, DerefMut};
use std::collections::VecDeque;

use ::{Error, Source, Sample};
use super::Buffer;

pub struct Buffered<'a> {
	source:  Source<'a>,
	buffers: VecDeque<Buffer>,
}

impl<'a> Buffered<'a> {
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

impl<'a> Deref for Buffered<'a> {
	type Target = Source<'a>;

	fn deref(&self) -> &<Self as Deref>::Target {
		&self.source
	}
}

impl<'a> DerefMut for Buffered<'a> {
	fn deref_mut(&mut self) -> &mut<Self as Deref>::Target {
		&mut self.source
	}
}

impl<'a> Drop for Buffered<'a> {
	fn drop(&mut self) {
		self.buffers.clear();
	}
}
