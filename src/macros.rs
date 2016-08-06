#[macro_export]
macro_rules! hybrid {

	($r:pat | $($i:ident $e:expr, $n:ident => $b:expr),*,) => ({
		hybrid!($r | $($i $e, $n => $b),*)
	});

	($r:pat | $($i:ident $e:expr, $n:ident => $b:expr),*) => ({
		use $crate::log::*;
		use $crate::ext::*;
		use $crate::db::*;
		use $crate::resptime::*;
		use iron::{BeforeMiddleware,
		           Chain, headers, modifiers, Response, status, typemap};
		use slog::Logger;
		use std::rc::Rc;
		use std::sync::Arc;

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
					db: req.extensions.get::<Db>().cloned(),
					log: req.ext::<Log>().clone(),
					rev: req.ext::<RevRoutes>().clone(),
				};
				match (req, elems) {
					$r => $b,
				}
			}
		};
		)*
		let mut chain = Chain::new(router! { $( $i $e => $n),* });
		let log = Log::setup_logger(Log::get_loglevel("SLOG_LEVEL"));
		let worklog = log.new(o![]);
		chain.link_before(Log::new(worklog));
		chain.link_before(Db);
		chain.link_before(revroutes);
		let mut chain = Chain::new(chain);
		chain.link_around(RespTime);

		let mut mount = Mount::new();
		let filepath = "files/";
		mount.mount("/", chain)
			.mount(&("/".to_owned() + filepath), Static::new(Path::new(filepath)));
		mount
	});

}
