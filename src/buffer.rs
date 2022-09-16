use crate::ResBuf;

pub trait BufferTrait {
	fn write(&self, what: &str);
}

impl<'a> BufferTrait for ResBuf<'a> {
	fn write(&self, what: &str) {
		self.push(what);
	}
}