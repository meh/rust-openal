use std::mem;
use std::marker::PhantomData;

use ffi::*;
use ::{Error, Vector, Position, Direction, Velocity, Buffer};
use super::{State, Offset, Stream};

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
	pub fn stream(self) -> Stream<'a> {
		unsafe {
			Stream::new(self)
		}
	}

	pub fn state(&self) -> State {
		unsafe {
			let mut state = 0;
			alGetSourcei(self.id, AL_SOURCE_STATE, &mut state);

			State::from(state)
		}
	}

	pub fn play(&mut self) {
		unsafe {
			alSourcePlay(self.id);
		}
	}

	pub fn pause(&mut self) {
		unsafe {
			alSourcePause(self.id);
		}
	}

	pub fn stop(&mut self) {
		unsafe {
			alSourceStop(self.id);
		}
	}

	pub fn rewind(&mut self) {
		unsafe {
			alSourceRewind(self.id);
		}
	}

	pub fn cone_inner_angle(&self) -> f32 {
		unsafe {
			let mut value = 0.0;
			alGetSourcef(self.id, AL_CONE_INNER_ANGLE, &mut value);

			value as f32
		}
	}

	pub fn set_cone_inner_angle(&mut self, value: f32) {
		unsafe {
			alSourcef(self.id, AL_CONE_INNER_ANGLE, value as ALfloat);
		}
	}

	pub fn cone_outer_angle(&self) -> f32 {
		unsafe {
			let mut value = 0.0;
			alGetSourcef(self.id, AL_CONE_OUTER_ANGLE, &mut value);

			value as f32
		}
	}

	pub fn set_cone_outer_angle(&mut self, value: f32) {
		unsafe {
			alSourcef(self.id, AL_CONE_OUTER_ANGLE, value as ALfloat);
		}
	}

	pub fn pitch(&self) -> f32 {
		unsafe {
			let mut value = 0.0;
			alGetSourcef(self.id, AL_PITCH, &mut value);

			value as f32
		}
	}

	pub fn set_pitch(&mut self, value: f32) {
		unsafe {
			alSourcef(self.id, AL_PITCH, value as ALfloat);
		}
	}

	pub fn position(&self) -> Position {
		unsafe {
			let mut value = Position(Vector { x: 0.0, y: 0.0, z: 0.0 });
			alGetSourcefv(self.id, AL_POSITION, mem::transmute(&mut value));

			value
		}
	}

	pub fn set_position(&self, value: &Position) {
		unsafe {
			alSourcefv(self.id, AL_POSITION, mem::transmute(value));
		}
	}

	pub fn direction(&self) -> Direction {
		unsafe {
			let mut value = Direction(Vector { x: 0.0, y: 0.0, z: 0.0 });
			alGetSourcefv(self.id, AL_DIRECTION, mem::transmute(&mut value));

			value
		}
	}

	pub fn set_direction(&self, value: &Direction) {
		unsafe {
			alSourcefv(self.id, AL_DIRECTION, mem::transmute(value));
		}
	}

	pub fn velocity(&self) -> Velocity {
		unsafe {
			let mut value = Velocity(Vector { x: 0.0, y: 0.0, z: 0.0 });
			alGetSourcefv(self.id, AL_VELOCITY, mem::transmute(&mut value));

			value
		}
	}

	pub fn set_velocity(&self, value: &Velocity) {
		unsafe {
			alSourcefv(self.id, AL_VELOCITY, mem::transmute(value));
		}
	}

	pub fn is_relative(&self) -> bool {
		unsafe {
			let mut value = 0;
			alGetSourcei(self.id, AL_SOURCE_RELATIVE, &mut value);

			value != AL_FALSE as ALint
		}
	}

	pub fn enable_relative(&mut self) {
		unsafe {
			alSourcei(self.id, AL_SOURCE_RELATIVE, AL_TRUE as ALint);
		}
	}

	pub fn disable_relative(&mut self) {
		unsafe {
			alSourcei(self.id, AL_SOURCE_RELATIVE, AL_FALSE as ALint);
		}
	}

	pub fn is_looping(&self) -> bool {
		unsafe {
			let mut value = 0;
			alGetSourcei(self.id, AL_LOOPING, &mut value);

			value != AL_FALSE as ALint
		}
	}

	pub fn enable_looping(&mut self) {
		unsafe {
			alSourcei(self.id, AL_LOOPING, AL_TRUE as ALint);
		}
	}

	pub fn disable_looping(&mut self) {
		unsafe {
			alSourcei(self.id, AL_LOOPING, AL_FALSE as ALint);
		}
	}

	pub fn gain(&self) -> f32 {
		unsafe {
			let mut value = 0.0;
			alGetSourcef(self.id, AL_GAIN, &mut value);

			value as f32
		}
	}

	pub fn set_gain(&mut self, value: f32) {
		unsafe {
			alSourcef(self.id, AL_GAIN, value as ALfloat);
		}
	}

	pub fn min_gain(&self) -> f32 {
		unsafe {
			let mut value = 0.0;
			alGetSourcef(self.id, AL_MIN_GAIN, &mut value);

			value as f32
		}
	}

	pub fn set_min_gain(&mut self, value: f32) {
		unsafe {
			alSourcef(self.id, AL_MIN_GAIN, value as ALfloat);
		}
	}

	pub fn max_gain(&self) -> f32 {
		unsafe {
			let mut value = 0.0;
			alGetSourcef(self.id, AL_MAX_GAIN, &mut value);

			value as f32
		}
	}

	pub fn set_max_gain(&mut self, value: f32) {
		unsafe {
			alSourcef(self.id, AL_MAX_GAIN, value as ALfloat);
		}
	}

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

	pub fn queued(&self) -> usize {
		unsafe {
			let mut value = 0;
			alGetSourcei(self.id, AL_BUFFERS_QUEUED, &mut value);

			value as usize
		}
	}

	pub fn processed(&self) -> usize {
		unsafe {
			let mut value = 0;
			alGetSourcei(self.id, AL_BUFFERS_PROCESSED, &mut value);

			value as usize
		}
	}

	pub fn push<'b: 'a>(&'b mut self, buffer: &'b Buffer<'b>) -> Result<(), Error> {
		unsafe {
			al_try!(alSourceQueueBuffers(self.id, 1, &buffer.id()));

			Ok(())
		}
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
