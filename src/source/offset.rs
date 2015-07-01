/// Represents the offset withing a `Source`.
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Offset {
	/// The offset is represented in seconds.
	Seconds(f32),

	/// The offset is represented in samples.
	Samples(f32),

	/// The offset is represented in bytes.
	Bytes(f32),
}

impl Offset {
	/// Get the offset in seconds.
	pub fn seconds(value: f32) -> Self {
		Offset::Seconds(value)
	}

	/// Get the offset in samples.
	pub fn samples(value: f32) -> Self {
		Offset::Samples(value)
	}

	/// Get the offset in bytes.
	pub fn bytes(value: f32) -> Self {
		Offset::Bytes(value)
	}

	/// Return a type hint for `Source::offset`.
	pub fn as_seconds() -> Self {
		Offset::Seconds(0.0)
	}

	/// Return a type hint for `Source::offset`.
	pub fn as_samples() -> Self {
		Offset::Samples(0.0)
	}

	/// Return a type hint for `Source::offset`.
	pub fn as_bytes() -> Self {
		Offset::Bytes(0.0)
	}
}
