use prelude::*;
use std;

mod views;

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

pub fn get_loglevel(env: &str) -> Level {
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
		Ok(val) => lvlc![&val[..], Trace, Debug, Info, Warning, Error],
		Err(_) => Level::Info,
	}
}

pub fn setup_logger(level: Level) -> Logger {
	let automatic = o!["line" => {
			|rec: &RecordInfo| {
				rec.line()
			}
		}, "mod" => {
			|rec: &RecordInfo| {
				rec.module().to_owned()
			}
		}];

	let log;
	if stderr_isatty() {
		log = drain::filter_level(level, ::slog_term::async_stderr()).into_logger(automatic);
		trace!(log, "Using drain", "out" => "stderr",
			"stderr_isatty" => stderr_isatty(),
			"type" => "term");
	} else {
		log = drain::filter_level(level,
		                          drain::async_stream(std::io::stderr(), ::slog_json::new()))
			.into_logger(automatic);
		trace!(log, "Using drain", "out" => "stderr",
			"stderr_isatty" => stderr_isatty(),
			"type" => "json");
	}
	log
}
