// https://atcoder.jp/contests/arc109/tasks/arc109_a

use std::cmp::min;

pub fn run(a: isize, b: isize, x: isize, y: isize) -> isize {
    if a < b {
        x + (b - a) * min(2 * x, y)
    } else if a > b {
        x + (a - b - 1) * min(2 * x, y)
    } else {
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize, isize, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 1, 1, 5, 1),
            TestCase(1, 2, 100, 1, 101),
        ];

        for TestCase(a, b, x, y, expected) in tests {
            assert_eq!(run(a, b, x, y), expected);
        }
    }
}
