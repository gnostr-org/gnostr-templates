#[allow(unused_variables)]
pub(crate) fn option() {
    let opt = Some("hello");

    let unwrapped = match opt {
        Some(str) => str,
        None => panic!("I'm panicking!"),
    };

    println!("opt: {:?}", opt);
    println!("unrwapped: {:?}", unwrapped);

    let some_u8_value = Some(0u8);
    #[allow(clippy::single_match)]
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(3) = some_u8_value {
        println!("three");
    }

    let res = play_with_question_mark();
}

fn play_with_question_mark() -> Result<String, String> {
    let res: Result<String, String> = Err("faaaail".to_string());

    // either unwrap or return the error
    let unwrap = res?;

    Ok(unwrap)
}
