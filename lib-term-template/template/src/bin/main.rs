extern crate chrono;
#[macro_use]
extern crate serde_derive;
extern crate termion;
extern crate tui;

extern crate lib_term;
use lib_term::*;

mod client;
mod messages;
mod server;

fn main() {
    let mut args = ::std::env::args();
    args.next();

    if let Some(name) = args.next() {
        lib_term::client::connect(client::App::new(name)).unwrap();
    } else {
        lib_term::server::spawn_shell_and_listen(server::App()).unwrap();
    }
}
