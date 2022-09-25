//! Defines the Buffer trait.

use crate::ResBuf;

pub trait BufferTrait<'a> {
	fn write(&mut self, what: &'a str);
}

impl<'a> BufferTrait<'a> for ResBuf<'a> {
	#[inline]
	fn write(&mut self, what: &'a str) {
		self.push(what);
	}
}