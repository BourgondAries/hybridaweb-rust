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
		struct Etuor {
			$(
				$n: &'static str
			),*
		}
		struct Etuors(Arc<Etuor>);
		impl typemap::Key for Etuors {
			type Value = Arc<Etuor>;
		}
		impl BeforeMiddleware for Etuors {
			fn before(&self, req: &mut Request) -> IronResult<()> {
				ins!(req, Etuors: self.0.clone());
				Ok(())
			}
		}
		let nak = Etuors(Arc::new(Etuor {
			$(
				$n: $e
			),*
		}));
		$(
		let $n = {
			|req: &mut Request| -> IronResult<Response> {
				let log = {
					req.ext::<Log>().clone()
				};
				let nak = {
					req.ext::<Etuors>().clone()
				};
				match match (req, log, nak) {
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

