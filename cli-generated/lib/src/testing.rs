pub(crate) fn testing() {
    println!("hello there");
}

#[allow(dead_code)]
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

#[allow(dead_code)]
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }

    #[test]
    fn it_works() {
        assert_eq!(add_two(2), 4);
    }

    #[test]
    #[ignore]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    #[should_panic]
    fn should_panic() {
        vec![1][100];
    }

    #[allow(clippy::eq_op)] // I think this is a false positive
    #[test]
    fn it_works_but_with_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
