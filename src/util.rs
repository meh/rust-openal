#[derive(PartialEq, Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Vector {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

#[derive(PartialEq, Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Position(pub Vector);

#[derive(PartialEq, Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Direction(pub Vector);

#[derive(PartialEq, Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Velocity(pub Vector);

#[derive(PartialEq, Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Orientation(pub Vector, pub Vector);
