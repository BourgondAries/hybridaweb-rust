#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]
#![feature(plugin)]
#![plugin(maud_macros)]

extern crate iron;
extern crate isatty;
extern crate itertools;
#[macro_use]
extern crate lazy_static;
extern crate maud;
extern crate mount;
extern crate postgres;
#[macro_use]
extern crate router;
#[macro_use(defer)]
extern crate scopeguard;
#[macro_use]
extern crate slog;
extern crate slog_json;
extern crate slog_term;
extern crate staticfile;
extern crate time;

mod ware;
pub mod db;
pub mod ext;
pub mod log;
pub mod resptime;
pub mod prelude;
#[macro_use]
pub mod macros;
