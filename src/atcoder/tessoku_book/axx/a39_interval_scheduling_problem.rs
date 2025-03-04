// https://atcoder.jp/contests/tessoku-book/tasks/math_and_algorithm_bn

use itertools::Itertools;

fn run(_n: usize, lr: Vec<(usize, usize)>) -> usize {
    let vec: Vec<(usize, usize)> = lr.into_iter().sorted_unstable_by(|a, b| a.1.cmp(&(b.1))).collect();

    let mut ans = 0;

    let mut cur = 0;

    for (l, r) in vec.into_iter() {
        if cur <= l {
            ans += 1;
            cur = r
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, usize)>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![(123, 86399), (1, 86400), (86399, 86400)], 2),
        ];

        for TestCase(n, lr, expected) in tests {
            assert_eq!(run(n, lr), expected);
        }
    }
}
