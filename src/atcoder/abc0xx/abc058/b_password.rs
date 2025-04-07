// https://atcoder.jp/contests/abc058/tasks/abc058_b

use itertools::{Itertools, EitherOrBoth::*};

fn run(o: &str, e: &str) -> String {
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

    struct TestCase(&'static str, &'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("xyz", "abc", "xaybzc"),
            TestCase("a", "b", "ab"),
            TestCase("atcoderbeginnercontest", "atcoderregularcontest", "aattccooddeerrbreeggiunlnaerrccoonntteesstt"),
        ];

        for TestCase(o, e, expected) in tests {
            assert_eq!(run(o, e), expected);
        }
    }
}
