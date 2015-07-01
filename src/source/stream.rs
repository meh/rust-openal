use std::ops::{Deref, DerefMut};
use std::collections::VecDeque;

use ffi::*;
use ::{Error, Source, Sample, Buffer};

/// Represents a streaming source for the `Listener`.
///
/// Stream sources will have buffers getting in and out of them, they're useful
/// for playing music and other kind of streaming sounds.
pub struct Stream<'a> {
	source:  Source<'a>,
	buffers: VecDeque<Buffer<'a>>,
}

impl<'a> Stream<'a> {
	#[doc(hidden)]
	pub unsafe fn new(mut source: Source) -> Stream {
		source.disable_looping();

		Stream {
			source:  source,
			buffers: VecDeque::new(),
		}
	}
}

impl<'a> Stream<'a> {
	/// Pushes the data into the source.
	pub fn push<T: Sample>(&mut self, channels: u16, data: &[T], rate: u32) -> Result<(), Error> {
		let buffer = try!(unsafe { Buffer::new(channels, data, rate) });
		try!(self.clear());

		unsafe {
			al_try!(alSourceQueueBuffers(self.id(), 1, &buffer.id()));
		}

		self.buffers.push_back(buffer);

		Ok(())
	}

	/// Removes a buffer from the front of the queue.
	pub fn pop(&mut self) -> Result<Buffer, Error> {
		if let Some(buffer) = self.buffers.pop_back() {
			unsafe {
				let mut tmp = 0;
				alSourceUnqueueBuffers(self.id(), 1 as ALsizei, &mut tmp);
			}

			if let Some(error) = Error::last() {
				self.buffers.push_front(buffer);

				Err(error)
			}
			else {
				Ok(buffer)
			}
		}
		else {
			Err(Error::InvalidOperation)
		}
	}

	/// Removes the processed buffers from the queue.
	pub fn clear(&mut self) -> Result<(), Error> {
		let processed = self.processed();

		if processed > 0 {
			unsafe {
				let mut tmp = vec![0; processed];
				al_try!(alSourceUnqueueBuffers(self.id(), processed as ALsizei, tmp.as_mut_ptr()));
			}

			for _ in 0 .. processed {
				self.buffers.pop_front();
			}
		}

		Ok(())
	}

	#[doc(hidden)]
	pub fn enable_looping(&mut self) -> ! {
		unreachable!();
	}

	#[doc(hidden)]
	pub fn disable_looping(&mut self) -> ! {
		unreachable!();
	}
}

impl<'a> Deref for Stream<'a> {
	type Target = Source<'a>;

	fn deref(&self) -> &<Self as Deref>::Target {
		&self.source
	}
}

impl<'a> DerefMut for Stream<'a> {
	fn deref_mut(&mut self) -> &mut<Self as Deref>::Target {
		&mut self.source
	}
}

impl<'a> ::std::fmt::Debug for Stream<'a> {
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
		try!(f.write_str("openal::source::Stream("));
		try!(f.write_str(&format!("{}; ", unsafe { self.id() })));
		try!(f.write_str(&format!("len={}", self.buffers.len())));
		f.write_str(")")
	}
}
