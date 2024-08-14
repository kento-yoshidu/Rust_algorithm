// https://atcoder.jp/contests/abc175/tasks/abc175_c

use std::cmp::min;

pub fn run(x: isize, k: isize, d: isize) -> isize {
    let mut xx = x.abs();
    let mut kk = k;

    let tmp = min(k, xx / d);

    kk -= tmp;

    xx -= tmp * d;

    if kk % 2 == 0 {
        xx
    } else {
        d - xx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(6, 2, 4, 2),
            TestCase(7, 4, 3, 1),
            TestCase(10, 1, 2, 8),
            TestCase(1000000000000000, 1000000000000000, 1000000000000000, 1000000000000000),
        ];

        for TestCase(x, k, d, expected) in tests {
            assert_eq!(run(x, k, d), expected);
        }
    }
}
