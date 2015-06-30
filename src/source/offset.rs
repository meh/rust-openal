#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Offset {
	Seconds(f32),
	Samples(f32),
	Bytes(f32),
}

impl Offset {
	pub fn seconds() -> Self {
		Offset::Seconds(0.0)
	}

	pub fn samples() -> Self {
		Offset::Samples(0.0)
	}

	pub fn bytes() -> Self {
		Offset::Bytes(0.0)
	}
}
