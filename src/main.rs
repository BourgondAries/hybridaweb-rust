#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

extern crate iron;
extern crate isatty;
extern crate itertools;
#[macro_use]
extern crate lazy_static;
extern crate postgres;
#[macro_use(defer)]
extern crate scopeguard;
#[macro_use]
extern crate slog;
extern crate slog_json;
extern crate slog_term;
extern crate time;

use iron::prelude::*;
use iron::{BeforeMiddleware, AfterMiddleware, AroundMiddleware, Handler, typemap};
use isatty::{stderr_isatty};
use itertools::Itertools;
use postgres::{Connection, UserInfo, ConnectParams, ConnectTarget, SslMode};
use scopeguard::guard;
use slog::*;
use std::collections::BTreeMap;
use std::env;
use std::io::{self, BufRead};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use std::thread::sleep;
use time::precise_time_ns;

struct Log(Arc<Logger>);
impl typemap::Key for Log { type Value = Arc<Logger>; }

impl BeforeMiddleware for Log {
	fn before(&self, req: &mut Request) -> IronResult<()> {
		req.extensions.insert::<Log>(self.0.clone());
		Ok(())
	}
}

struct ResponseTimeHandler(Box<Handler>);
struct ResponseTime;

macro_rules! elog {
	($i:ident) => { $i.extensions.get::<Log>().unwrap() }
}

impl Handler for ResponseTimeHandler {
	fn handle(&self, req: &mut Request) -> IronResult<Response> {
		let begin = precise_time_ns();
		let response = self.0.handle(req);
		let delta = precise_time_ns() - begin;
		let conn = Connection::connect("postgresql://postgres:@localhost", SslMode::None)
			.map_err(|x| {
				elog!(req).trace("Wrong!", b!["error" => format!("{:?}", x)]);
			});
		elog!(req).trace("Request time", b![
			"ms" => delta / 1000 / 1000, "us" => delta / 1000 % 1000, "ns" => delta % 1000
		]);
		if let Ok(conn) = conn {

		}
		response
	}
}

impl AroundMiddleware for ResponseTime {
	fn around(self, handler: Box<Handler>) -> Box<Handler> {
		Box::new(ResponseTimeHandler(handler))
	}
}

fn hello_world(req: &mut Request) -> IronResult<Response> {
	elog!(req).info("", b!["req" => format!("{:?}", req)]);
	sleep(Duration::new(1, 0));
	Ok(Response::with((iron::status::Ok, "Hello World")))
}

macro_rules! matchfor {
	($e:expr; $($p:pat => $b:expr)*) => {{
		for _1 in $e {
			match *_1 {
				$(
					$p => $b
				),*
			}
		}
	}};
}

fn main() {
	let log = setup_logger(get_loglevel("SLOG_LEVEL"));
	let mainlog = log.new(o!["thread" => "main"]);
	let worklog = Arc::new(log.new(o![]));

	defer!(mainlog.trace("Clean exit", b![]));
	mainlog.trace("Constructing middleware", b![]);

	let mut chain = Chain::new(hello_world);
	chain.link_before(Log(worklog));
	chain.link_around(ResponseTime);
	Iron::new(chain).http("localhost:3000").unwrap();
}

fn get_loglevel(env: &str) -> Level {
	macro_rules! lvlc {
		($n:expr, $($i:ident),*) => {{
			match $n {
				$(
					stringify!($i) => Level::$i,
				)*
				_ => Level::Info,
			}
		}};
	}
	match env::var(env) {
		Ok(val) => {
			lvlc![&val[..], Trace, Debug, Info, Warning, Error]
		}
		Err(_) => Level::Info,
	}
}

fn setup_logger(level: Level) -> Logger {
	let log = Logger::new_root(o!());

	if ! stderr_isatty() {
		log.set_drain(
			drain::filter_level(
				level,
				drain::async_stream(
					std::io::stderr(),
					slog_json::new(),
				),
			),
		);
		log.trace("Using drain", b!("out" => "stderr", "stderr_isatty" => stderr_isatty(), "type" => "json"));
	} else {
		log.set_drain(
			drain::filter_level(
				level,
				slog_term::async_stderr()
			)
		);
		log.trace("Using drain", b!("out" => "stderr", "stderr_isatty" => stderr_isatty(), "type" => "term"));
	}
	log
}

