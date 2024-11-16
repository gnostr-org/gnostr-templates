use std::io;
use std::io::Write;

pub(crate) fn fibonacci() {
    println!("How many numbers in the Fibonacci sequence?");

    let mut count = String::new();

    io::stdin()
        .read_line(&mut count)
        .expect("Failed to read line");

    let count: u32 = count.trim().parse().expect("Please type a number!");

    fibonacci_n(count)
}

fn fibonacci_n(n: u32) {
    for number in 1..n {
        print!("{} ", fib2(number));
        io::stdout().flush().unwrap();
    }
    println!()
}

#[allow(dead_code)]
fn fib(n: u32) -> u32 {
    if n == 1 {
        return 1;
    }

    if n == 2 {
        return 1;
    }

    fib(n - 1) + fib(n - 2)
}

fn fib2(n: u32) -> u32 {
    match n {
        1 => 1,
        2 => 1,
        _ => fib2(n - 1) + fib2(n - 2),
    }
}
