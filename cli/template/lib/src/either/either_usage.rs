use crate::either::either_usage::Either::{Left, Right};

pub(crate) fn either() {
    println!("either!");

    println!(
        "This is a Left: {:?}",
        Left::<String, String>("vanster".to_string())
    );
    println!(
        "This is a Right: {:?}",
        Right::<String, String>("hoger".to_string())
    );
}

#[derive(Debug)]
enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    #[inline]
    pub fn map<U, F: FnOnce(R) -> U>(self, f: F) -> Either<L, U> {
        match self {
            Left(l) => Left(l),
            Right(r) => Right(f(r)),
        }
    }

    pub fn unwrap(self) -> R {
        match self {
            Left(_) => panic!("Can't unwrap on a left either"),
            Right(r) => r,
        }
    }

    pub fn unwrap_left(self) -> L {
        match self {
            Left(l) => l,
            Right(_) => panic!("Can't unwrap_left on a right either"),
        }
    }

    pub fn to_result(self) -> Result<R, L> {
        match self {
            Left(l) => Err(l),
            Right(r) => Ok(r),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

    #[test]
    fn it_should_unwrap() {
        let either = Right::<&str, &str>("hello");
        let unwrapped = either.unwrap();
        assert_eq!("hello", unwrapped);
    }

    #[should_panic]
    #[test]
    fn it_should_panic_when_unwrapping_left() {
        let either = Left::<&str, &str>("hello");
        let unwrapped = either.unwrap();
    }

    #[test]
    fn it_unwraps_left() {
        let either = Left::<&str, &str>("hello");
        let unwrapped = either.unwrap_left();
        assert_eq!("hello", unwrapped);
    }

    #[should_panic]
    #[test]
    fn it_should_panic_when_unwrapping_left_on_a_right() {
        let either = Right::<&str, &str>("hello");
        let unwrapped = either.unwrap_left();
    }

    #[test]
    fn it_maps_right_either() {
        let either = Right::<&str, &str>("hello");
        let mapped = either.map(|r| r.len());

        assert_eq!(5, mapped.unwrap());
    }

    #[test]
    fn right_should_convert_to_ok_result() {
        let either = Right::<&str, &str>("hello");
        let result = either.to_result();
        assert_eq!("hello", result.unwrap());
    }

    #[test]
    fn left_should_convert_to_err_result() {
        let either = Left::<&str, &str>("hello");
        let result = either.to_result();
        assert_eq!("hello", result.unwrap_err());
    }
}
