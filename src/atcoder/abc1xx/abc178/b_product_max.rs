// https://atcoder.jp/contests/abc178/tasks/abc178_b

use std::cmp::max;

fn run(a: isize, b: isize, c: isize, d: isize) -> isize {
    max(max(a*c, a*d), max(b*c, b*d))
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize, isize, isize);

    #[test]
    fn abc178_b() {
        let tests = [
            TestCase(1, 2, 1, 1, 2),
            TestCase(3, 5, -4, -2, -6),
            TestCase(-1000000000, 0, -1000000000, 0, 1000000000000000000),
        ];

        for TestCase(a, b, c, d, expected) in tests {
            assert_eq!(run(a, b, c, d), expected);
        }
    }
}
