use prelude::*;

pub trait SetCookie {
	fn cookie<K: ToString, V: ToString>(mut self, key: K, value: V) -> Self;
}

impl SetCookie for IronResult<Response> {
	fn cookie<K: ToString, V: ToString>(mut self, key: K, value: V) -> Self {
		if let Ok(ref mut resp) = self {
			{
				if let Some(ref mut cookies) = resp.headers.get_mut::<Cookie>() {
					cookies.push(CookiePair::new(key.to_string(), value.to_string()));
				}
			}
			{
				if ! resp.headers.has::<Cookie>() {
					resp.headers.set(
						Cookie(vec![
							CookiePair::new(key.to_string(), value.to_string())
						])
					);
				}
			}
		}
		self
	}
}
