use ffi::*;

/// Representes the state of the source.
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum State {
	/// Unknown state, something is most likely wrong.
	Unknown,

	/// Initial state of the source.
	Initial,

	/// The source is playing.
	Playing,

	/// The source is paused.
	Paused,

	/// The source is stopped.
	Stopped,
}

impl From<ALenum> for State {
	fn from(value: ALint) -> State {
		match value {
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
