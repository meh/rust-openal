#![feature(core)]

extern crate libc;
extern crate openal_sys as ffi;

pub mod error;
pub use error::Error;

use std::ffi::CStr;
use std::str::from_utf8_unchecked;

use ffi::*;

pub fn vendor() -> Option<&'static str> {
	unsafe {
		let ptr = alGetString(AL_VENDOR);

		if ptr.is_null() {
			None
		}
		else {
			Some(from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()))
		}
	}
}

pub fn version() -> Option<&'static str> {
	unsafe {
		let ptr = alGetString(AL_VERSION);

		if ptr.is_null() {
			None
		}
		else {
			Some(from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()))
		}
	}
}

pub fn renderer() -> Option<&'static str> {
	unsafe {
		let ptr = alGetString(AL_RENDERER);

		if ptr.is_null() {
			None
		}
		else {
			Some(from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()))
		}
	}
}

pub fn extensions() -> Option<&'static str> {
	unsafe {
		let ptr = alGetString(AL_EXTENSIONS);

		if ptr.is_null() {
			None
		}
		else {
			Some(from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()))
		}
	}
}
