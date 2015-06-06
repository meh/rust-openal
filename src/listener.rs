use std::mem;

use ffi::*;
use ::{Vector, Position, Velocity, Orientation};

pub fn gain() -> f32 {
	unsafe {
		let mut value = 0.0;
		alGetListenerf(AL_GAIN, &mut value);

		value as f32
	}
}

pub fn set_gain(value: f32) {
	unsafe {
		alListenerf(AL_GAIN, value as ALfloat);
	}
}

pub fn position() -> Position {
	unsafe {
		let mut value = Position(Vector { x: 0.0, y: 0.0, z: 0.0 });
		alGetListenerfv(AL_POSITION, mem::transmute(&mut value));

		value
	}
}

pub fn set_position(value: &Position) {
	unsafe {
		alListenerfv(AL_POSITION, mem::transmute(value));
	}
}

pub fn velocity() -> Velocity {
	unsafe {
		let mut value = Velocity(Vector { x: 0.0, y: 0.0, z: 0.0 });
		alGetListenerfv(AL_VELOCITY, mem::transmute(&mut value));

		value
	}
}

pub fn set_velocity(value: &Velocity) {
	unsafe {
		alListenerfv(AL_VELOCITY, mem::transmute(value));
	}
}

pub fn orientation() -> Orientation {
	unsafe {
		let mut value = Orientation(Vector { x: 0.0, y: 0.0, z: 0.0 }, Vector { x: 0.0, y: 0.0, z: 0.0 });
		alGetListenerfv(AL_ORIENTATION, mem::transmute(&mut value));

		value
	}
}
