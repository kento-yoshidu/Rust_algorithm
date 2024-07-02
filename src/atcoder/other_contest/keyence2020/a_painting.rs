// https://atcoder.jp/contests/keyence2020/tasks/keyence2020_a

use std::cmp::max;

fn run(h: usize, w: usize, n: usize) -> usize {
    let l = max(h, w);

    if n % l == 0 {
        n / l
    } else {
        n / l + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 7, 10, 2),
            TestCase(14, 12, 112, 8),
            TestCase(2, 100, 200, 2),
        ];

        for TestCase(h, w, n, expected) in tests {
            assert_eq!(run(h, w, n), expected);
        }
    }
}
