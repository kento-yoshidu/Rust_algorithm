// https://atcoder.jp/contests/abc297/tasks/abc297_d

use std::cmp::{min, max};

fn rec(a: usize, b: usize) -> usize {
    if b == 0 {
        0
    } else {
        a / b + rec(b, a % b)
    }
}

fn run(a: usize, b: usize) -> usize {
    let l = max(a, b);
    let s = min(a, b);

    rec(l, s) - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn abc297_d() {
        let tests = [
            TestCase(3, 8, 4),
            TestCase(1234567890, 1234567890, 0),
            TestCase(1597, 987, 15),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
