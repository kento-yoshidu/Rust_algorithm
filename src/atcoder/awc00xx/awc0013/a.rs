// https://atcoder.jp/contests/awc0013/tasks/awc0013_a

use std::cmp::max;

fn run(a: isize, b: isize, c: isize) -> isize {
    max(0, b - a) * c
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize, isize);

    #[test]
    fn awc0013_a() {
        let tests = [
            TestCase(50, 100, 3, 150),
            TestCase(150, 100, 5, 0),
            TestCase(999999000, 1000000000, 100, 100000),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
