#[macro_export]
macro_rules! hybrid {

	($($i:ident $e:expr, $n:ident : $r:pat => $b:expr),*,) => ({
		hybrid!($($i $e, $n : $r => $b),*)
	});

	($($i:ident $e:expr, $n:ident : $r:pat => $b:expr),*) => ({
		use $crate::log::*;
		use $crate::ext::*;
		use $crate::db::*;
		use $crate::reply::*;
		use $crate::resptime::*;
		use $crate::htmlize::*;
		use iron::{AfterMiddleware, AroundMiddleware, BeforeMiddleware,
		           Chain, headers, modifiers, Response, status, typemap};
		use slog::Logger;
		use std::rc::Rc;
		use std::sync::Arc;

		type Surrounder = Arc<fn(String) -> String>;
		struct HybridChain {
			chain: Chain,
			surround: Surrounder,
		}

		impl HybridChain {
			fn surround(&mut self, sur: Surrounder) {
				self.surround = sur;
			}
		}

		impl Handler for HybridChain {
			fn handle(&self, req: &mut Request) -> IronResult<Response> {
				self.chain.handle(req)
			}
		}

		#[allow(dead_code)]
		struct RevRoute { $( $n: &'static str),* }
		struct RevRoutes(Arc<RevRoute>);
		impl typemap::Key for RevRoutes { type Value = Arc<RevRoute>; }
		impl BeforeMiddleware for RevRoutes {
			fn before(&self, req: &mut Request) -> IronResult<()> {
				req.ins::<RevRoutes>(self.0.clone());
				Ok(())
			}
		}
		let revroutes = RevRoutes(Arc::new(RevRoute { $( $n: $e),* }));
		$(
		let $n = {
			|req: &mut Request| -> IronResult<Response> {
				#[allow(dead_code)]
				struct Elements {
					db: Option<Rc<Connection>>,
					log: Arc<Logger>,
					rev: Arc<RevRoute>,
				}
				let elems = Elements {
					db: req.extensions.get::<Db>().map(|x| x.clone()),
					log: req.ext::<Log>().clone(),
					rev: req.ext::<RevRoutes>().clone(),
				};
				match match (req, elems) {
					$r => $b,
				} {
					Reply::Html(out)
						=> Ok(Response::with((status::Ok, out))),
					Reply::Redirect(out)
						=> Ok(Response::with((status::Found, modifiers::Header(headers::Location(out))))),
				}
			}
		};
		)*
		let mut chain = Chain::new(router! { $( $i $e => $n),* });
		let log = Log::setup_logger(Log::get_loglevel("SLOG_LEVEL"));
		let mainlog = log.new(o!["reqid" => "main"]);
		let worklog = log.new(o![]);
		chain.link_before(Log::new(worklog));
		chain.link_before(Db);
		chain.link_before(revroutes);
		chain.link_after(Htmlize);
		let mut chain = Chain::new(chain);
		chain.link_around(RespTime);
		HybridChain {
			chain: chain,
			surround: Arc::new(|x: String| -> String { x }),
		}
	});

}
