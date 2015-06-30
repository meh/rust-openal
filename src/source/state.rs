use ffi::*;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum State {
	Unknown,
	Initial,
	Playing,
	Paused,
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
