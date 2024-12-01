// https://atcoder.jp/contests/abc214/tasks/abc214_c

use std::cmp::min;

fn run(n: usize, s: Vec<usize>, t: Vec<usize>) -> Vec<usize> {
    let mut ans = vec![std::usize::MAX; n];

    ans[0] = min(s[0], t[0]);

    for i in 1..n*2 {
        ans[i%n] = min(ans[(i-1)%n] + s[(i-1)%n], t[i%n]);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<usize>, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![4, 1, 5], vec![3, 10, 100], vec![3, 7, 8]),
            TestCase(4, vec![100, 100, 100, 100], vec![1, 1, 1, 1], vec![1, 1, 1, 1]),
            TestCase(4, vec![1, 2, 3, 4], vec![1, 2, 4, 7], vec![1, 2, 4, 7]),
            TestCase(8, vec![84, 87, 78, 16, 94, 36, 87, 93], vec![50, 22, 63, 28, 91, 60, 64, 27], vec![50, 22, 63, 28, 44, 60, 64, 27]),
        ];

        for TestCase(n, s, t, expected) in tests {
            assert_eq!(run(n, s, t), expected);
        }
    }
}
