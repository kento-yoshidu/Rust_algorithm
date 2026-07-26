// https://atcoder.jp/contests/abc326/tasks/abc326_c

use itertools::Itertools;

fn run(n: usize, m: usize, a: Vec<usize>) -> usize {
    let a: Vec<usize> = a.into_iter().sorted().collect();

    let mut ans = 0;

    for i in 0..n {
        let upper = a.partition_point(|&x| x < a[i] + m);
        ans = ans.max(upper - i);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn abc326_c() {
        let tests = [
            TestCase(8, 6, vec![2, 3, 5, 7, 11, 13, 17, 19], 4),
            TestCase(10, 1, vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3], 2),
            TestCase(10, 998244353, vec![100000007, 0, 1755647, 998244353, 495, 1000000000, 1755648, 503, 1755649, 998244853], 7),
        ];

        for TestCase(n, m, a, expected) in tests {
            assert_eq!(run(n, m, a), expected);
        }
    }
}
