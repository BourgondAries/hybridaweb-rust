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
		$(
			let $n = |req: &mut Request| -> IronResult<Response> {
				match (req, 1) {
					$r => $b,
				}
			};
		)*
		router! {
			$(
				$i $e => $n
			),*
		}
	});
}

