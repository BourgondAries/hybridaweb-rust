use prelude::*;

pub struct Db;

impl typemap::Key for Db {
	type Value = Rc<Connection>;
}

impl BeforeMiddleware for Db {
	fn before(&self, req: &mut Request) -> IronResult<()> {
		let path = "postgresql://postgres:abc@localhost/hybrida";
		debug![req.ext::<Log>(), "Connecting to database", "path" => path];
		let conn = Connection::connect(path, SslMode::None).map_err(|x| {
			crit![req.ext::<Log>(), "Unable to connec to db", "error" => format!("{:?}", x)];
		});
		if let Ok(conn) = conn {
			req.ins::<Db>(Rc::new(conn));
		}
		Ok(())
	}
}
