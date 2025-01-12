// https://atcoder.jp/contests/abc070/tasks/abc070_b

use std::cmp::{min, max};

fn run(a: isize, b: isize, c: isize, d: isize) -> isize {
    let start = max(a, c);
    let end = min(b, d);

    max(0, end-start)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize, isize, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(0, 75, 25, 100, 50),
            TestCase(0, 33, 66, 99, 0),
            TestCase(10, 90, 20, 80, 60),
        ];

        for TestCase(a, b, c, d, expected) in tests {
            assert_eq!(run(a, b, c, d), expected);
        }
    }
}
