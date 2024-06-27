// https://atcoder.jp/contests/nikkei2019-qual/tasks/nikkei2019_qual_a

use std::cmp::{max, min};

fn run(n: isize, a: isize, b: isize) -> (isize, isize) {
    (min(a, b), max(0, a + b - n))
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize, (isize, isize));

    #[test]
    fn test() {
        let tests = [
            TestCase(10, 3, 5, (3, 0)),
            TestCase(10, 7, 5, (5, 2)),
            TestCase(100, 100, 100, (100, 100)),
        ];

        for TestCase(n, a, b, expexted) in tests {
            assert_eq!(run(n, a, b), expexted);
        }
    }
}
