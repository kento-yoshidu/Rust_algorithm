// https://atcoder.jp/contests/abc428/tasks/abc428_a

use std::cmp::min;

fn run(s: usize, a: usize, b: usize, x: usize) -> usize {
    let rem = x % (a + b);
    let sum = x / (a + b) * a * s;

    sum + min(a, rem) * s
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, usize);

    #[test]
    fn abc428_a() {
        let tests = [
            TestCase(7, 3, 2, 11, 49),
            TestCase(6, 3, 2, 9, 36),
            TestCase(1, 1, 666, 428, 1),
        ];

        for TestCase(s, a, b, x, expected) in tests {
            assert_eq!(run(s, a, b, x), expected);
        }
    }
}
