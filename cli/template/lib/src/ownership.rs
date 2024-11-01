#[allow(unused_variables)]
pub(crate) fn ownership() {
    let mut str = String::from("hello");

    reference(&str);
    mutable(&mut str);

    test(str);

    #[allow(unused_mut)]
    let mut mutable = String::from("hi");
    let immutable = mutable.clone();
    //    immutable.push_str("!");

    let hello = String::from("hi");

    test(hello)
}

fn test(str: String) {
    println!("test{:?}", str);
}

fn reference(str: &str) {
    println!("ref {:?}", str);
}

fn mutable(str: &mut String) {
    str.push_str("!");
    println!("mut: {:?}", str);
}
