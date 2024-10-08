// https://atcoder.jp/contests/abc183/tasks/abc183_c

use itertools::Itertools;

fn run(n: usize, k: usize, t: Vec<Vec<usize>>) -> usize {
    let mut ans = 0;

    for p in (1..n).permutations(n-1) {
        let mut tmp = 0;

        for arr in p.windows(2) {
            tmp += t[arr[0]][arr[1]];
        }

        tmp += t[0][p[0]];
        tmp += t[p[n-2]][0];

        if tmp == k {
            ans += 1;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<Vec<usize>>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, 330, vec![vec![0, 1, 10, 100], vec![1, 0, 20, 200], vec![10, 20, 0, 300], vec![100, 200, 300, 0]], 2),
            TestCase(5, 5, vec![vec![0, 1, 1, 1, 1], vec![1, 0, 1, 1, 1], vec![1, 1, 0, 1, 1], vec![1, 1, 1, 0, 1], vec![1, 1, 1, 1, 0]], 24),
        ];

        for TestCase(n, k, t, expected) in tests {
            assert_eq!(run(n, k, t), expected);
        }
    }
}
