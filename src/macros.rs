macro_rules! elog {
	($i:ident) => { $i.extensions.get::<Log>().unwrap() }
}

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
macro_rules! req {
	($($i:ident $e:expr, $n:ident : $r:pat => $b:expr),*,) => ({
		req!($($i $e, $n : $r => $b),*)
	});
	($($i:ident $e:expr, $n:ident : $r:pat => $b:expr),*) => ({
		#[allow(dead_code)]
		struct RevRoute {
			$(
				$n: &'static str
			),*
		}
		struct RevRoutes(Arc<RevRoute>);
		impl typemap::Key for RevRoutes {
			type Value = Arc<RevRoute>;
		}
		impl BeforeMiddleware for RevRoutes {
			fn before(&self, req: &mut Request) -> IronResult<()> {
				ins!(req, RevRoutes: self.0.clone());
				Ok(())
			}
		}
		let nak = RevRoutes(Arc::new(RevRoute {
			$(
				$n: $e
			),*
		}));
		$(
		let $n = {
			|req: &mut Request| -> IronResult<Response> {
				let log = req.ext::<Log>().clone();
				let nak = req.ext::<RevRoutes>().clone();
				let db = req.ext::<Db>().clone();
				match match (req, log, nak, db) {
					$r => $b,
				} {
					Re::Html(out) => Ok(Response::with((status::Ok, out))),
					Re::Redirect(out) => Ok(Response::with((status::Found, Header(headers::Location(out))))),
				}
			}
		};
		)*
		let mut chain = Chain::new(router! {
			$(
				$i $e => $n
			),*
		});
		chain.link_before(nak);
		chain
	});
}

