// https://atcoder.jp/contests/joi2025yo1a/tasks/joi2025_yo1a_d

use std::cmp::max;

fn run(_n: usize, _m: usize, a: Vec<usize>, b: Vec<usize>) -> usize {
    let mut ans = 0;

    for x in a.iter() {
        for y in b.iter() {
            ans += (x+y) * max(x, y);
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 2, vec![1, 2], vec![2, 5], 79),
            TestCase(1, 5, vec![50], vec![9, 7, 5, 4, 1], 13800),
            TestCase(15, 5, vec![5, 10, 52, 31, 14, 16, 19, 1, 9, 20, 80, 19, 11, 34, 72], vec![20, 2, 4, 9, 19], 116756),
        ];

        for TestCase(n, m, a, b, expected) in tests {
            assert_eq!(run(n, m, a, b), expected);
        }
    }
}
