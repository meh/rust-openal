#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Offset {
	Seconds(f32),
	Samples(f32),
	Bytes(f32),
}

impl Offset {
	pub fn seconds(value: f32) -> Self {
		Offset::Seconds(value)
	}

	pub fn samples(value: f32) -> Self {
		Offset::Samples(value)
	}

	pub fn bytes(value: f32) -> Self {
		Offset::Bytes(value)
	}

	pub fn as_seconds() -> Self {
		Offset::Seconds(0.0)
	}

	pub fn as_samples() -> Self {
		Offset::Samples(0.0)
	}

	pub fn as_bytes() -> Self {
		Offset::Bytes(0.0)
	}
}
