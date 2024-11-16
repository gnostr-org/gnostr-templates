#[allow(unused_variables)]
pub(crate) fn variables() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);

    let x = five();

    println!("The value of five() is: {}", x);

    let hello = 1;
    let hello = "qweqwe";
}

fn five() -> i32 {
    5
}
