// https://atcoder.jp/contests/abc448/tasks/abc448_b

use std::cmp::max;

fn run(_n: usize, _m: usize, c: Vec<isize>, ab: Vec<(usize, isize)>) -> isize {
    let mut ans = 0;

    let mut c = c.clone();

    for (a, b) in ab {
        if c[a-1] >= b {
            ans += b;
        } else {
            ans += c[a-1];
        }

        c[a-1] = max(0, c[a-1] - b);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<isize>, Vec<(usize, isize)>, isize);

    #[test]
    fn abc448_b() {
        let tests = [
            TestCase(7, 5, vec![4, 4, 8, 3, 7], vec![(1, 2), (2, 3), (5, 2), (4, 10), (2, 3), (5, 4), (2, 3)], 15),
            TestCase(1, 1, vec![1], vec![(1, 1)], 1),
            TestCase(15, 10, vec![7, 94, 100, 82, 63, 81, 75, 2, 76, 73], vec![(10, 44), (5, 77), (10, 47), (7, 32), (2, 82), (5, 90), (3, 37), (6, 70), (6, 28), (3, 25), (2, 26), (10, 56), (1, 69), (5, 46), (7, 26)], 438)
        ];

        for TestCase(n, m, c, ab, expected) in tests {
            assert_eq!(run(n, m, c, ab), expected);
        }
    }
}
