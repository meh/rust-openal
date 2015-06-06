use std::mem;

use ffi::*;
use ::{Error, Buffer, Vector, Position, Direction, Velocity};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum State {
	Unknown,
	Initial,
	Playing,
	Paused,
	Stopped,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Mode {
	Static,
	Streaming,
	Undetermined,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Offset {
	Seconds(f32),
	Samples(f32),
	Bytes(f32),
}

#[derive(PartialEq, Eq)]
pub struct Source {
	id: ALuint,
}

impl Source {
	pub unsafe fn id(&self) -> ALuint {
		self.id
	}
}

impl Source {
	pub fn new() -> Self {
		unsafe {
			let mut id = 0;
			alGenSources(1, &mut id);

			Source { id: id }
		}
	}

	pub fn mode(&self) -> Mode {
		unsafe {
			let mut mode = 0;
			alGetSourcei(self.id, AL_SOURCE_TYPE, &mut mode);

			match mode {
				AL_STATIC =>
					Mode::Static,

				AL_STREAMING =>
					Mode::Streaming,

				_ =>
					Mode::Undetermined
			}
		}
	}

	pub fn set_mode(&mut self, mode: Mode) {
		unsafe {
			let mode = match mode {
				Mode::Static       => AL_STATIC,
				Mode::Streaming    => AL_STREAMING,
				Mode::Undetermined => AL_UNDETERMINED,
			};

			alSourcei(self.id, AL_SOURCE_TYPE, mode);
		}
	}

	pub fn state(&self) -> State {
		unsafe {
			let mut state = 0;
			alGetSourcei(self.id, AL_SOURCE_STATE, &mut state);

			match state {
				AL_INITIAL =>
					State::Initial,

				AL_PLAYING =>
					State::Playing,

				AL_PAUSED =>
					State::Paused,

				AL_STOPPED =>
					State::Stopped,

				_ =>
					State::Unknown
			}
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
	
	pub fn offset(&self, kind: Offset) -> Offset {
		unsafe {
			let e = match kind {
				Offset::Seconds(..) =>
					AL_SEC_OFFSET,
				
				Offset::Samples(..) =>
					AL_SAMPLE_OFFSET,

				Offset::Bytes(..) =>
					AL_BYTE_OFFSET
			};

			let mut value = 0.0;
			alGetSourcef(self.id, e, &mut value);

			match kind {
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

	pub fn push<'s, 'b: 's>(&'s mut self, buffer: &'b Buffer) -> Queued<'b> {
		Queued::new(self, buffer)
	}
}

impl Drop for Source {
	fn drop(&mut self) {
		unsafe {
			alDeleteSources(1, &self.id);

			if let Some(error) = Error::last() {
				panic!("{}", error)
			}
		}
	}
}

#[must_use]
pub struct Queued<'a> {
	source: ALuint,
	buffer: &'a Buffer,
}

impl<'a> Queued<'a> {
	pub fn new<'s, 'b: 's>(source: &'s Source, buffer: &'b Buffer) -> Queued<'b> {
		unsafe {
			alSourceQueueBuffers(source.id(), 1, &buffer.id());

			Queued { source: source.id(), buffer: buffer }
		}
	}
}

impl<'a> Drop for Queued<'a> {
	fn drop(&mut self) {
		unsafe {
			alSourceUnqueueBuffers(self.source, 1, &self.buffer.id());
		}
	}
}
