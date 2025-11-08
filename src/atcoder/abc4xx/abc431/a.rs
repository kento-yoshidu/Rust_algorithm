// https://atcoder.jp/contests/abc431/tasks/abc431_a

use std::cmp::max;

fn run(h: isize, b: isize) -> isize {
    max(0, h - b)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize);

    #[test]
    fn abc431_a() {
        let tests = [
            TestCase(43, 1, 42),
            TestCase(4, 31, 0),
            TestCase(1, 1, 0),
        ];

        for TestCase(h, b, expected) in tests {
            assert_eq!(run(h, b), expected);
        }
    }
}
