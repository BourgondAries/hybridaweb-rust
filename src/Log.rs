use prelude::*;

pub struct Log(Arc<Logger>, Mutex<u64>);

impl Log {
	pub fn new(log: Logger) -> Log {
		Log(Arc::new(log), Mutex::new(0))
	}
}

impl typemap::Key for Log {
	type Value = Arc<Logger>;
}

impl BeforeMiddleware for Log {
	fn before(&self, req: &mut Request) -> IronResult<()> {
		let reqid = {
			let mut count = self.1.lock().unwrap();
			*count = count.wrapping_add(1);
			*count
		};
		req.ins::<Log>(Arc::new(self.0.new(o!["reqid" => reqid])));
		trace![req.ext::<Log>(), "Beginning request"];
		Ok(())
	}
}
