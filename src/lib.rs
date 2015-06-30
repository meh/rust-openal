#![allow(raw_pointer_derive)]

extern crate libc;
extern crate openal_sys as ffi;

#[macro_use]
pub mod util;
pub use util::{Vector, Position, Direction, Velocity, Orientation, Doppler};

pub mod traits;
pub use traits::{Device, Context};

pub mod error;
pub use error::Error;

pub mod listener;
pub use listener::Listener;

pub mod capture;
pub use capture::Capture;

pub mod source;
pub use source::Source;

pub mod buffer;
pub use buffer::Buffer;

pub mod extension;

pub mod sample;
pub use sample::Sample;
