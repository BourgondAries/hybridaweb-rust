use prelude::*;
use std::net::ToSocketAddrs;
use iron::error::HttpResult;
use iron::Listening;

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

/*
pub trait FindPort {
	fn http_randport<A: ToSocketAddrs>(self, addr: A) -> HttpResult<Listening>;
}

impl<H: Handler> FindPort for Iron<H> {
	fn http_randport<A: ToSocketAddrs>(self, addr: A) -> HttpResult<Listening> {
		let mut count = 1000;
		let max = u16::max_value();
		loop {
			let current = &(format!["localhost:{}", count])[..];
			let server = self.http(current);
			if let Err(err) = server {
				if count == max {
					return server;
				}
				count += 1;
			} else {
				return server;
			}
		}
	}
}
*/
