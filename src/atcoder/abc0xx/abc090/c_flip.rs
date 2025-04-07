// https://atcoder.jp/contests/abc090/tasks/arc091_a

use std::cmp::{min, max};

fn run(n: usize, m: usize) -> usize {
    let n = min(n, m);
    let m = max(n, m);

    if n == 1 {
        if m == 1 {
            1
        } else {
            m - 2
        }
    } else {
        (n - 2) * (m - 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 2, 0),
            TestCase(1, 7, 5),
            TestCase(314, 1592, 496080),
        ];

        for TestCase(n, m, expected) in tests {
            assert_eq!(run(n, m), expected);
        }
    }
}
