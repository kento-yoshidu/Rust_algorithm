// https://atcoder.jp/contests/joi2018yo/tasks/joi2018_yo_a

use std::cmp::min;

fn run(n: usize, a: usize, b: usize, c: usize, d: usize) -> usize {
    let x = (n + a - 1) / a;
    let y = (n + c - 1) / c;

    min(x*b, y*d)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(10, 3, 100, 5, 180, 360),
            TestCase(6, 2, 200, 3, 300, 600),
        ];

        for TestCase(n, a, b, c, d, expected) in tests {
            assert_eq!(run(n, a, b, c, d), expected);
        }
    }
}
