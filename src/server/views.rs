use slog::Logger;

pub fn index(log: &Logger) -> String {
	trace![log, "Generating html"];
	let mut buffer = String::new();
	match html! {
		buffer,
		html {
			head {
				meta charset="UTF-8" /
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
