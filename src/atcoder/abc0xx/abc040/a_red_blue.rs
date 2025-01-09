// https://atcoder.jp/contests/abc040/tasks/abc040_a

use std::cmp::min;

fn run(n: usize, x: usize) -> usize {
    min(n - x, x - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, 2, 1),
            TestCase(6, 4, 2),
            TestCase(90, 30, 29),
        ];

        for TestCase(n, x, expected) in tests {
            assert_eq!(run(n, x), expected);
        }
    }
}
