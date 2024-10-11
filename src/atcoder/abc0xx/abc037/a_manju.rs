// https://atcoder.jp/contests/abc037/tasks/abc037_a

use std::cmp::min;

fn run(a: usize, b: usize, c: usize) -> usize {
    c / min(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 5, 6, 2),
            TestCase(8, 6, 20, 3),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
