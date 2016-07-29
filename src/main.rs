#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

extern crate iron;
extern crate isatty;
extern crate itertools;
#[macro_use]
extern crate lazy_static;
extern crate mount;
extern crate postgres;
#[macro_use]
extern crate router;
#[macro_use(defer)]
extern crate scopeguard;
#[macro_use]
extern crate slog;
extern crate slog_json;
extern crate slog_term;
extern crate staticfile;
extern crate time;

use iron::prelude::*;
use iron::{BeforeMiddleware, AfterMiddleware, AroundMiddleware, Handler, typemap};
use isatty::{stderr_isatty};
// use itertools::Itertools;
use mount::Mount;
use postgres::{Connection, SslMode};
// use scopeguard::guard;
use slog::*;
// use std::collections::BTreeMap;
use staticfile::Static;
use std::env;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread::sleep;
use time::precise_time_ns;

macro_rules! elog {
	($i:ident) => { $i.extensions.get::<Log>().unwrap() }
}

macro_rules! ins {
	($i:ident, $t:ty, $e:expr) => {{
		$i.extensions.insert::<$t>($e)
	}};
}

macro_rules! ext {
	($i:ident, $t:ty) => { $i.extensions.get::<$t>().unwrap() }
}

struct Log(Arc<Logger>, Mutex<u64>);

impl Log {
	fn new(log: Logger) -> Log {
		Log(Arc::new(log), Mutex::new(0))
	}
}

impl typemap::Key for Log { type Value = Arc<Logger>; }

impl BeforeMiddleware for Log {
	fn before(&self, req: &mut Request) -> IronResult<()> {
		let reqid = {
			let mut count = self.1.lock().unwrap();
			*count = count.wrapping_add(1);
			*count
		};
		ins!(req, Log, Arc::new(self.0.new(o!["reqid" => reqid])));
		elog!(req).trace("Beginning request", b![]);
		Ok(())
	}
}

struct ResponseTime;
impl AroundMiddleware for ResponseTime {
	fn around(self, handler: Box<Handler>) -> Box<Handler> {
		Box::new(ResponseTimeHandler(handler))
	}
}

struct ResponseTimeHandler(Box<Handler>);
impl Handler for ResponseTimeHandler {
	fn handle(&self, req: &mut Request) -> IronResult<Response> {
		let begin = precise_time_ns();
		let response = self.0.handle(req);
		let delta = precise_time_ns() - begin;
		let conn = Connection::connect("postgresql://postgres:abc@localhost/hybrida", SslMode::None)
			.map_err(|x| {
				elog!(req).critical("Unable to connec to db", b!["error" => format!("{:?}", x)]);
			});
		if let Ok(conn) = conn {
			let _ = conn.transaction();
		}

		elog!(req).trace("Request time", b![
			"ms" => delta / 1000 / 1000, "us" => delta / 1000 % 1000, "ns" => delta % 1000
		]);

		response
	}
}

fn hello_world(req: &mut Request) -> IronResult<Response> {
	elog!(req).info("", b!["req" => format!("{:?}", req)]);
	sleep(Duration::new(0, 1000*1000*200));
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
	let mainlog = log.new(o!["reqid" => "main"]);
	let worklog = log.new(o![]);

	defer!(mainlog.trace("Clean exit", b![]));
	mainlog.trace("Constructing middleware", b![]);

	let router = router! {
		get  ""       => hello_world,
	};

	let mut chain = Chain::new(router);
	chain.link_before(Log::new(worklog));
	chain.link_around(ResponseTime);

	let mut mount = Mount::new();
	mount
		.mount("/dl/", Static::new(Path::new("target/debug/")))
		.mount("/", chain)
	;

	mainlog.trace("Firing up server", b![]);
	let _ = Iron::new(mount).http("localhost:3000").map_err(|x| {
		mainlog.error("Unable to start server", b!["error" => format!("{:?}", x)]);
	});
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

