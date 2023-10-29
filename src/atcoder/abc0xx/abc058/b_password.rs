// https://atcoder.jp/contests/abc058/tasks/abc058_b

use itertools::{Itertools, EitherOrBoth::*};

pub fn run(o: String, e: String) -> String {
    o.chars()
        .zip_longest(e.chars())
        .map(|t| {
            match t {
                Both(l, r) => format!("{}{}", l, r),
                Left(l) => format!("{}", l),
                Right(r) => format!("{}", r),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("xaybzc"), run(String::from("xyz"), String::from("abc")));
        assert_eq!(String::from("ab"), run(String::from("a"), String::from("b")));
        assert_eq!(String::from("aattccooddeerrbreeggiunlnaerrccoonntteesstt"), run(String::from("atcoderbeginnercontest"), String::from("atcoderregularcontest")));
    }
}
