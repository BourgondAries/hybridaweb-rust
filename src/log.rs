use prelude::*;
use std::io;

pub struct Log(Arc<Logger>, Mutex<u64>);

impl Log {
	pub fn new(log: Logger) -> Log {
		Log(Arc::new(log), Mutex::new(0))
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
			log = drain::filter_level(level, drain::async_stream(io::stderr(), ::slog_json::new()))
				.into_logger(automatic);
			trace!(log, "Using drain", "out" => "stderr",
				"stderr_isatty" => stderr_isatty(),
				"type" => "json");
		}
		log
	}
}

impl typemap::Key for Log {
	type Value = Arc<Logger>;
}

impl BeforeMiddleware for Log {
	fn before(&self, req: &mut Request) -> IronResult<()> {
		let reqid = {
			// TODO: use unwrap_or_else with an RNG
			let mut count = self.1.lock().unwrap();
			*count = count.wrapping_add(1);
			*count
		};
		req.ins::<Log>(Arc::new(self.0.new(o!["reqid" => reqid])));
		trace![req.ext::<Log>(), "Beginning request"];
		Ok(())
	}
}
