#[allow(unused_variables)]
pub fn slice() {
    let whole_word = String::from("hello world");

    let word = first_word_old(&whole_word);
    let word = first_word(&whole_word);

    println!("{} -> {}", whole_word, word);

    //String to slice
    let test = &String::from("hello")[..];
    println!("Test: {}", test);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_old(s: &str) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
