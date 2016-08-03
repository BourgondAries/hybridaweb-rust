use prelude::*;

pub trait Ext<'a> {
	fn ext<T: typemap::Key>(&'a self) -> &'a T::Value;
	fn ins<T: typemap::Key>(&mut self, val: T::Value);
}

impl<'a, 'b> Ext<'a> for Request<'a, 'b> {
	fn ext<T: typemap::Key>(&'a self) -> &'a T::Value {
		self.extensions.get::<T>().unwrap()
	}
	fn ins<T: typemap::Key>(&mut self, val: T::Value) {
		self.extensions.insert::<T>(val);
	}
}
