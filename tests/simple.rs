#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]
#![feature(plugin)]
#![plugin(maud_macros)]

#[macro_use]
extern crate hybridweb;
extern crate hyper;
extern crate iron;
extern crate maud;
#[macro_use(router)]
extern crate router;
#[macro_use]
extern crate slog;

use hybridweb::prelude::*;
use hyper::client::{Client};
use std::io::Read;

const STANDARD_SERVER : &'static str = "localhost:3000";

fn checkbody(request: &str, expect_body: &str) {
	let client = Client::new();
		let mut response = client.get(&format!["http://{}/{}", STANDARD_SERVER, request]).send().unwrap();
		let mut string = String::new();
		let _ = response.read_to_string(&mut string);
		assert_eq![string, expect_body]
}

macro_rules! revroute {
	($($e:expr; $i:ident);*) => ({
		#[allow(non_camel_case_types)]
		pub struct Temporary;
		impl Temporary {
			pub fn route(&self) -> String {
				let mut string = String::new();
				$(
					string += $e;
					string += stringify![$i];
				)*
				string
			}
			pub fn encode<T: ToString>(&self, $($i: T),*) -> String {
				let mut string = String::new();
				$(
					string += $e;
					string += "/";
					string += $i.to_string().as_ref();
				)*
				string
			}
		}
		Temporary
	});
}

#[test]
fn main() {

	let temp = revroute!["control"; user];
	println!["{}", temp.encode("something")];
	let temp = revroute!["control"; seonc];
	println!["{}", temp.encode("something completely different")];

	const CONTROL_VALUE: &'static str = "control value";

	let hybrid = hybrid! {
		(req, elm) |
		get "/", example_route => {
			rep![CONTROL_VALUE]
		},
	};

	let mut result = Iron::new(hybrid).http(STANDARD_SERVER).unwrap();
	for _ in 0..10 {
		checkbody("", CONTROL_VALUE);
	}
	let _ = result.close();
}
