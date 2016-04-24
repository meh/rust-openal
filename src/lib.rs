/*!
OpenAL wrapper for Rust.
*/

#![warn(missing_docs)]

extern crate libc;
extern crate openal_sys as ffi;

#[macro_use]
mod util;
pub use util::{Vector, Position, Direction, Velocity, Orientation, Doppler, Sample};

mod traits;
pub use traits::{Device, Context};

mod error;
pub use error::Error;

pub mod listener;
pub use listener::Listener;

pub mod capture;
pub use capture::Capture;

pub mod source;
pub use source::Source;

mod buffer;
pub use buffer::Buffer;

pub mod extension;
