macro_rules! elog {
	($i:ident) => { $i.extensions.get::<Log>().unwrap() }
}

#[macro_export]
macro_rules! ins {
	($i:ident, $t:ty: $e:expr) => {{
		$i.extensions.insert::<$t>($e)
	}};
}

macro_rules! ext {
	($i:ident, $t:ty) => { $i.extensions.get::<$t>().unwrap() }
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

#[macro_export]
macro_rules! req {
	($($i:ident $e:expr, $n:ident : $r:pat => $b:expr),*,) => ({
		req!($($i $e, $n : $r => $b),*)
	});
	($($i:ident $e:expr, $n:ident : $r:pat => $b:expr),*) => ({
		use $crate::server::{Db, Html, Log, Reply};
		use iron::{AfterMiddleware, AroundMiddleware, BeforeMiddleware, Chain, headers, modifiers, Response, status, typemap};
		use slog::Logger;
		use std::sync::Arc;
		use std::rc::Rc;

		#[allow(dead_code)]
		struct RevRoute { $( $n: &'static str),* }
		struct RevRoutes(Arc<RevRoute>);
		impl typemap::Key for RevRoutes { type Value = Arc<RevRoute>; }
		impl BeforeMiddleware for RevRoutes {
			fn before(&self, req: &mut Request) -> IronResult<()> {
				ins!(req, RevRoutes: self.0.clone());
				Ok(())
			}
		}
		let revroutes = RevRoutes(Arc::new(RevRoute { $( $n: $e),* }));
		$(
		let $n = {
			|req: &mut Request| -> IronResult<Response> {
				#[allow(dead_code)]
				struct Elements {
					db: Rc<Connection>,
					log: Arc<Logger>,
					rev: Arc<RevRoute>,
				}
				let elems = Elements {
					log: req.ext::<Log>().clone(),
					rev: req.ext::<RevRoutes>().clone(),
					db: req.ext::<Db>().clone(),
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
		chain.link_before(revroutes);
		chain
	});
}
