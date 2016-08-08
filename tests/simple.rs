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

const standard_server: &'static str = "localhost:3000";

fn checkbody(request: &str, expect_body: &str) {
	let client = Client::new();
		let mut response = client.get(&format!["http://{}/{}", standard_server, request]).send().unwrap();
		let mut string = String::new();
		let _ = response.read_to_string(&mut string);
		assert_eq![string, expect_body]
}

#[test]
fn main() {

	const control_value: &'static str = "control value";

	let hybrid = hybrid! {
		(req, elm) |
		get "/", example_route => {
			rep![control_value]
		},
	};

	let mut result = Iron::new(hybrid).http(standard_server);
	for _ in 0..10 {
		checkbody("", control_value);
	}
	result.map(|ref mut x| x.close());
}
