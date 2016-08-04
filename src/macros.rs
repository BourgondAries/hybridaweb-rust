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
		use std::cell::RefCell;
		use std::rc::Rc;
		use std::sync::{Arc, Mutex};

		type Surrounder = Arc<Mutex<RefCell<Box<fn(String) -> String>>>>;
		struct HybridChain {
			surround: Surrounder,
		}

		impl HybridChain {
			fn surround_with(&self, sur: fn(String) -> String) {
				let x = self.surround.clone();
				let x = x.lock().unwrap();
				let mut x = x.borrow_mut();
				*x = Box::new(sur);
			}
		}

		impl typemap::Key for HybridChain { type Value = Surrounder; }
		impl BeforeMiddleware for HybridChain {
			fn before(&self, req: &mut Request) -> IronResult<()> {
				req.ins::<HybridChain>(self.surround.clone());
				Ok(())
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
				let surround = req.ext::<HybridChain>().clone();
				let surround = surround.lock().unwrap();
				let surround = surround.borrow();
				match match (req, elems) {
					$r => $b,
				} {
					Reply::Html(out)
						=> Ok(Response::with((status::Ok, surround(out)))),
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
		fn default_surround(string: String) -> String { string };
		let surround = Arc::new(Mutex::new(RefCell::new(Box::new(default_surround as fn(String) -> String))));
		let hchain = Arc::new(HybridChain {
			surround: surround.clone(),
		});
		chain.link_before(hchain.clone());
		(chain, hchain)
	});

}
