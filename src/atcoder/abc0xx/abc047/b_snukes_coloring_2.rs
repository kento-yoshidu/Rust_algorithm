// https://atcoder.jp/contests/abc047/tasks/abc047_b

use std::cmp::max;

fn run(w: i32, h: i32, _n: i32, xya: Vec<(i32, i32, i32)>) -> i32 {
    let mut xt = h;
    let mut xb = 0;
    let mut yl = 0;
    let mut yr = w;

    for (x, y, a) in xya {
        match a {
            1 => yl = yl.max(x),
            2 => yr = yr.min(x),
            3 => xb = xb.max(y),
            4 => xt = xt.min(y),
            _ => unreachable!(),
        }
    }

    max(xt - xb, 0) * max(yr - yl, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(i32, i32, i32, Vec<(i32, i32, i32)>, i32);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, 4, 2, vec![(2, 1, 1), (3, 3, 4)], 9),
            TestCase(5, 4, 3, vec![(2, 1, 1), (3, 3, 4), (1, 4, 2)], 0),
            TestCase(10, 10, 5, vec![(1, 6, 1), (4, 1, 3), (6, 9, 4), (9, 4, 2), (3, 1, 3)], 64),
        ];

        for TestCase(w, h, n, xya, expected) in tests {
            assert_eq!(expected, run(w, h, n, xya));
        }
    }
}
