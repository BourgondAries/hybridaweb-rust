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

mod include;
#[macro_use]
mod macros;
mod server;

// Idea: use macros to automate contextual handler generation
// For example, a simple website has multiple tabs, one needs
// to register a tab at an index, associating it with a function.
// Each function needs to handle the request, a db connection, and
// a logger. These could all be added to the request:
//
// req.trace("Handler X is invoked", b![]);
// let trans = req.db.transaction();
// ...
//
// Ideally, each file invokes the following macro:
//
// req! {
// get "prefix/cool" fname: req => {
// req.trace("Handler X is invoked", b![]);
// let trans = req.db.transaction();
// }
// }
//
// But the req needs to register the entity at a handler. This must
// be automated. One giant req! could be created, but I'm not sure if this
// is desired. If we put all requests in their own modules, then
// we'll somehow need to collect them. This sounds eerily like a global
// static is needed. This must be avoided.
//
// The cool thing about every controller in the same file is that we
// can actually make the macro such that it records all function names
// inside a struct, and maps them to their prefixes, in addition to checking
// if none are the same. This is actually really nice, and forces the programmer
// to make the controllers small.
//
// req! {
// get "prefix/cool" fname: req => {
// req.trace("Handler X is invoked", b![]);
// let trans = req.db.transaction();
// }
// get "/" fname: req => {
// let trans = req.db.transaction();
// req.
// if let Some(uid) = req.uid() {
// login();
// }
// }
// }
//
// I like the idea, as it takes care of some boilerplate.
// This currently works for only one route, which should be fine.
// Other things to do is allow codes like /u/ in mount, which
// automatically logs you in (by checking a cookie).
//
// Another mount is /dl/, which downloads a static file.
// It's really nice not having any user stuff or db connection
// here.
//
// Other I now need to implement backreferences in addition to
// header code generation.
//

fn main() {
	server::enter();
}
