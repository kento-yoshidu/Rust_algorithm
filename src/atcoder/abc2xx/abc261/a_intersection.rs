// https://atcoder.jp/contests/abc261/tasks/abc261_a

use std::cmp::{min, max};

fn run(l1: isize, r1: isize, l2: isize, r2: isize) -> isize {
    max(0, min(r1, r2) - max(l1, l2))
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize, isize, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(0, 3, 1, 5, 2),
            TestCase(0, 1, 4, 5, 0),
            TestCase(0, 3, 3, 7, 0),
        ];

        for TestCase(l1, r1, l2, r2, expected) in tests {
            assert_eq!(run(l1, r1, l2, r2), expected);
        }
    }
}
