use slog::Logger;

pub fn index(log: &Logger) -> String {
	let mut buffer = String::new();
	match html! {
		buffer,
		html {
			head {
			}
			body {
				p {
					"Da fuk man"
				}
			}
		}
	} {
		Ok(()) => {},
		Err(_) => {},
	}
	buffer
}
