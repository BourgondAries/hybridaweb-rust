use prelude::*;

pub struct Htmlize;

impl AfterMiddleware for Htmlize {
	fn after(&self, req: &mut Request, mut res: Response) -> IronResult<Response> {
		trace![req.ext::<Log>(), "Setting MIME to html"];
		(Mime(TopLevel::Text, SubLevel::Html, vec![])).modify(&mut res);
		Ok(res)
	}
}


