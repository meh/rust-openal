use std::mem;
use std::marker::PhantomData;

use ffi::*;
use ::{Error, Vector, Position, Direction, Velocity, Buffer};
use super::{State, Offset, Stream};

/// Represents a static source.
///
/// Static sources will have buffers queued that will never change, they're
/// useful for sound effects and similar.
#[derive(PartialEq, Eq)]
pub struct Source<'a> {
	id: ALuint,

	_marker: PhantomData<&'a ()>,
}

impl<'a> Source<'a> {
	#[doc(hidden)]
	pub unsafe fn new() -> Result<Self, Error> {
		let mut id = 0;
		al_try!(alGenSources(1, &mut id));

		Ok(Source { id: id, _marker: PhantomData })
	}

	#[doc(hidden)]
	pub unsafe fn id(&self) -> ALuint {
		self.id
	}
}

impl<'a> Source<'a> {
	/// Returns a streaming source.
	pub fn stream(self) -> Stream<'a> {
		unsafe {
			Stream::new(self)
		}
	}

	/// Gets the state.
	pub fn state(&self) -> State {
		unsafe {
			let mut state = 0;
			alGetSourcei(self.id, AL_SOURCE_STATE, &mut state);

			State::from(state)
		}
	}

	/// Plays the source.
	pub fn play(&mut self) {
		unsafe {
			alSourcePlay(self.id);
		}
	}

	/// Pauses the source.
	pub fn pause(&mut self) {
		unsafe {
			alSourcePause(self.id);
		}
	}

	/// Stops the source.
	pub fn stop(&mut self) {
		unsafe {
			alSourceStop(self.id);
		}
	}

	/// Rewinds the source.
	pub fn rewind(&mut self) {
		unsafe {
			alSourceRewind(self.id);
		}
	}

	/// Gets the cone inner angle.
	pub fn cone_inner_angle(&self) -> f32 {
		unsafe {
			let mut value = 0.0;
			alGetSourcef(self.id, AL_CONE_INNER_ANGLE, &mut value);

			value as f32
		}
	}

	/// Sets the cone inner angle.
	pub fn set_cone_inner_angle(&mut self, value: f32) {
		unsafe {
			alSourcef(self.id, AL_CONE_INNER_ANGLE, value as ALfloat);
		}
	}

	/// Gets the cone outer angle.
	pub fn cone_outer_angle(&self) -> f32 {
		unsafe {
			let mut value = 0.0;
			alGetSourcef(self.id, AL_CONE_OUTER_ANGLE, &mut value);

			value as f32
		}
	}

	/// Sets the cone outer angle.
	pub fn set_cone_outer_angle(&mut self, value: f32) {
		unsafe {
			alSourcef(self.id, AL_CONE_OUTER_ANGLE, value as ALfloat);
		}
	}

	/// Gets the pitch.
	pub fn pitch(&self) -> f32 {
		unsafe {
			let mut value = 0.0;
			alGetSourcef(self.id, AL_PITCH, &mut value);

			value as f32
		}
	}

	/// Sets the pitch.
	pub fn set_pitch(&mut self, value: f32) {
		unsafe {
			alSourcef(self.id, AL_PITCH, value as ALfloat);
		}
	}

	/// Gets the position.
	pub fn position(&self) -> Position {
		unsafe {
			let mut value = Position(Vector { x: 0.0, y: 0.0, z: 0.0 });
			alGetSourcefv(self.id, AL_POSITION, mem::transmute(&mut value));

			value
		}
	}

	/// Sets the position.
	pub fn set_position(&self, value: &Position) {
		unsafe {
			alSourcefv(self.id, AL_POSITION, mem::transmute(value));
		}
	}

	/// Gets the direction.
	pub fn direction(&self) -> Direction {
		unsafe {
			let mut value = Direction(Vector { x: 0.0, y: 0.0, z: 0.0 });
			alGetSourcefv(self.id, AL_DIRECTION, mem::transmute(&mut value));

			value
		}
	}

	/// Sets the direction.
	pub fn set_direction(&self, value: &Direction) {
		unsafe {
			alSourcefv(self.id, AL_DIRECTION, mem::transmute(value));
		}
	}

	/// Gets the velocity.
	pub fn velocity(&self) -> Velocity {
		unsafe {
			let mut value = Velocity(Vector { x: 0.0, y: 0.0, z: 0.0 });
			alGetSourcefv(self.id, AL_VELOCITY, mem::transmute(&mut value));

			value
		}
	}

	/// Sets the velocity.
	pub fn set_velocity(&self, value: &Velocity) {
		unsafe {
			alSourcefv(self.id, AL_VELOCITY, mem::transmute(value));
		}
	}

	/// Checks if the source is relative.
	pub fn is_relative(&self) -> bool {
		unsafe {
			let mut value = 0;
			alGetSourcei(self.id, AL_SOURCE_RELATIVE, &mut value);

			value != AL_FALSE as ALint
		}
	}

	/// Makes the source relative.
	pub fn enable_relative(&mut self) {
		unsafe {
			alSourcei(self.id, AL_SOURCE_RELATIVE, AL_TRUE as ALint);
		}
	}

	/// Makes the source absolute.
	pub fn disable_relative(&mut self) {
		unsafe {
			alSourcei(self.id, AL_SOURCE_RELATIVE, AL_FALSE as ALint);
		}
	}

	/// Checks if the source is looping.
	pub fn is_looping(&self) -> bool {
		unsafe {
			let mut value = 0;
			alGetSourcei(self.id, AL_LOOPING, &mut value);

			value != AL_FALSE as ALint
		}
	}

	/// Makes the source looping.
	pub fn enable_looping(&mut self) {
		unsafe {
			alSourcei(self.id, AL_LOOPING, AL_TRUE as ALint);
		}
	}

	/// Makes the source one-shot.
	pub fn disable_looping(&mut self) {
		unsafe {
			alSourcei(self.id, AL_LOOPING, AL_FALSE as ALint);
		}
	}

	/// Gets the gain.
	pub fn gain(&self) -> f32 {
		unsafe {
			let mut value = 0.0;
			alGetSourcef(self.id, AL_GAIN, &mut value);

			value as f32
		}
	}

	/// Sets the gain.
	pub fn set_gain(&mut self, value: f32) {
		unsafe {
			alSourcef(self.id, AL_GAIN, value as ALfloat);
		}
	}

	/// Gets the minimum gain.
	pub fn min_gain(&self) -> f32 {
		unsafe {
			let mut value = 0.0;
			alGetSourcef(self.id, AL_MIN_GAIN, &mut value);

			value as f32
		}
	}

	/// Sets the minimum gain.
	pub fn set_min_gain(&mut self, value: f32) {
		unsafe {
			alSourcef(self.id, AL_MIN_GAIN, value as ALfloat);
		}
	}

	/// Gets the maximum gain.
	pub fn max_gain(&self) -> f32 {
		unsafe {
			let mut value = 0.0;
			alGetSourcef(self.id, AL_MAX_GAIN, &mut value);

			value as f32
		}
	}

	/// Sets the minimum gain.
	pub fn set_max_gain(&mut self, value: f32) {
		unsafe {
			alSourcef(self.id, AL_MAX_GAIN, value as ALfloat);
		}
	}

	/// Gets the offset in the specified representation.
	pub fn offset(&self, offset: Offset) -> Offset {
		unsafe {
			let kind = match offset {
				Offset::Seconds(..) =>
					AL_SEC_OFFSET,

				Offset::Samples(..) =>
					AL_SAMPLE_OFFSET,

				Offset::Bytes(..) =>
					AL_BYTE_OFFSET
			};

			let mut value = 0.0;
			alGetSourcef(self.id, kind, &mut value);

			match offset {
				Offset::Seconds(..) =>
					Offset::Seconds(value as f32),

				Offset::Samples(..) =>
					Offset::Samples(value as f32),

				Offset::Bytes(..) =>
					Offset::Bytes(value as f32)
			}
		}
	}

	/// Sets the offset.
	pub fn set_offset(&mut self, value: Offset) {
		unsafe {
			match value {
				Offset::Seconds(value) =>
					alSourcef(self.id, AL_SEC_OFFSET, value),

				Offset::Samples(value) =>
					alSourcef(self.id, AL_SAMPLE_OFFSET, value),

				Offset::Bytes(value) =>
					alSourcef(self.id, AL_BYTE_OFFSET, value)
			}
		}
	}

	/// Gets how many buffers are queued.
	pub fn queued(&self) -> usize {
		unsafe {
			let mut value = 0;
			alGetSourcei(self.id, AL_BUFFERS_QUEUED, &mut value);

			value as usize
		}
	}

	/// Gets how many buffers have been processed.
	pub fn processed(&self) -> usize {
		unsafe {
			let mut value = 0;
			alGetSourcei(self.id, AL_BUFFERS_PROCESSED, &mut value);

			value as usize
		}
	}

	/// Pushes a buffer into the source's queue.
	pub fn push<'b: 'a>(&'b mut self, buffer: &'b Buffer<'b>) -> Result<(), Error> {
		unsafe {
			al_try!(alSourceQueueBuffers(self.id, 1, &buffer.id()));

			Ok(())
		}
	}
}

impl<'a> ::std::fmt::Debug for Source<'a> {
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
		try!(f.write_str("openal::Source("));
		try!(f.write_str(&format!("{}", unsafe { self.id() })));
		f.write_str(")")
	}
}

impl<'a> Drop for Source<'a> {
	fn drop(&mut self) {
		if self.state() != State::Playing && self.state() != State::Paused {
			self.stop();
		}

		unsafe {
			alDeleteSources(1, &self.id);
			al_panic!();
		}
	}
}
