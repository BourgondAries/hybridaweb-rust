use prelude::*;

pub struct RespTime;

impl AroundMiddleware for RespTime {
	fn around(self, handler: Box<Handler>) -> Box<Handler> {
		Box::new(RespTimeHandler(handler))
	}
}

struct RespTimeHandler(Box<Handler>);

impl Handler for RespTimeHandler {
	fn handle(&self, req: &mut Request) -> IronResult<Response> {
		let begin = precise_time_ns();
		let response = self.0.handle(req);
		let delta = precise_time_ns() - begin;

		trace!(req.ext::<Log>(), "Request time",
			"ms" => delta / 1000 / 1000, "us" => delta / 1000 % 1000, "ns" => delta % 1000
		);

		response
	}
}
