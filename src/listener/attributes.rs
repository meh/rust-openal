use ffi::*;

#[derive(PartialEq, Eq, Copy, Clone, Default, Debug)]
pub struct Attributes {
	pub frequency:   Option<i32>,
	pub refresh:     Option<i32>,
	pub synchronous: bool,
	pub mono:        Option<i32>,
	pub stereo:      Option<i32>,
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
