extern crate chrono;
#[macro_use]
extern crate serde_derive;
extern crate termion;
extern crate tui;

extern crate {{project-name | snake_case}};
use {{project-name | snake_case}}::*;

mod client;
mod messages;
mod server;

fn main() {
    let mut args = ::std::env::args();
    args.next();

    if let Some(name) = args.next() {
        {{project-name | snake_case}}::client::connect(client::App::new(name)).unwrap();
    } else {
        {{project-name | snake_case}}::server::spawn_shell_and_listen(server::App()).unwrap();
    }
}
