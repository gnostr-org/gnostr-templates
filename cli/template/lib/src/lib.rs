use collections::collections;
use concurrency::thread2::thread2;
use concurrency::threads::threads;
use either::either_usage::either;
use expressions::expressions;
use fibonacci::fibonacci;
use guess::guess;
use json::json;
use lifetimes::lifetimes;
use loops::loops;
use making_lists::ring_buffer;
use notification::notification;
use option::option;
use ownership::ownership;
use rest::rest;
use slice::slice;
use smart_pointers::pointers::pointers;
use smart_pointers::refcell::refcell;
use smart_pointers::tree::tree;
use structs_rectangle::structs_rectangle;
use subdir::another_file::hello_from_another_dir;
use testing::testing;
use traits::traits;
use variables::variables;

use install::install;
pub mod install;

pub mod collections;
pub mod concurrency;
pub mod either;
pub mod expressions;
pub mod fibonacci;
pub mod guess;
pub mod json;
pub mod lifetimes;
pub mod loops;
pub mod making_lists;
pub mod notification;
pub mod option;
pub mod ownership;
pub mod rest;
pub mod slice;
pub mod smart_pointers;
pub mod structs_rectangle;
pub mod subdir;
pub mod testing;
pub mod traits;
pub mod variables;

///
/// Elaborate function to sum two integers
/// # Example
/// ```
/// let sum = gnostr_lib::add(1,3);
/// assert_eq!(4, sum);
/// ```

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

///
/// gnostr guess
/// # gnostr guess
/// ```
/// use std::process::Command;
/// let mut binding = Command::new("gnostr");
/// let gnostr_guess = binding.arg("guess");
/// print!("gnostr_guess={:?}", gnostr_guess);
/// ```

pub fn run(name: &str) {
    match name {
        "guess" => guess(),
        "variables" => variables(),
        "expressions" => expressions(),
        "loops" => loops(),
        "fibonacci" => fibonacci(),
        "ownership" => ownership(),
        "slice" => slice(),
        "rectangle" => structs_rectangle(),
        "option" => option(),
        "json" => json(),
        "collections" => collections(),
        "traits" => traits(),
        "lifetimes" => lifetimes(),
        "testing" => testing(),
        "subdir" => hello_from_another_dir(),
        "pointers" => pointers(),
        "refcell" => refcell(),
        "tree" => tree(),
        "threads" => threads(),
        "thread2" => thread2(),
        "either" => either(),
        "notification" => notification(),
        "rest" => rest(),
        "lists" => ring_buffer::ring_buffer(),
        "install" => install(),
        _ => panic!("Invalid option"),
    };
}
