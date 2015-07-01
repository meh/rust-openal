use ffi::*;

/// Represents the attributes that can be set when a `Listener` is opened.
#[derive(PartialEq, Eq, Copy, Clone, Default, Debug)]
pub struct Attributes {
	/// Frequency for mixing output buffer, in units of Hz.
	pub frequency: Option<i32>,

	/// Refresh intervals, in units of Hz.
	pub refresh: Option<i32>,

	/// Flag indicating a synchronous `Listener`.
	pub synchronous: bool,

	/// A hint indicating how many `Source`s should be capable of supporting mono
	/// data.
	pub mono: Option<i32>,

	/// A hint indicating how many `Source`s should be capable of supporting
	/// stereo data.
	pub stereo: Option<i32>,
}

impl<'a> From<&'a Attributes> for Vec<ALint> {
	fn from(attributes: &Attributes) -> Vec<ALint> {
		let mut result = Vec::new();

		if let Some(value) = attributes.frequency {
			result.push(ALC_FREQUENCY);
			result.push(value);
		}

		if let Some(value) = attributes.refresh {
			result.push(ALC_REFRESH);
			result.push(value);
		}

		if attributes.synchronous {
			result.push(ALC_SYNC);
			result.push(ALC_TRUE as ALint);
		}

		if let Some(value) = attributes.mono {
			result.push(ALC_MONO_SOURCES);
			result.push(value);
		}

		if let Some(value) = attributes.stereo {
			result.push(ALC_STEREO_SOURCES);
			result.push(value);
		}

		result.push(0);

		result
	}
}
