// https://atcoder.jp/contests/abc263/tasks/abc263_c

use itertools::Itertools;

fn run(n: usize, m: usize) -> Vec<Vec<usize>> {
    let mut ans = Vec::new();

    for a in (1..=m).combinations(n) {
        ans.push(a);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<Vec<usize>>);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 3, vec![vec![1, 2], vec![1, 3], vec![2, 3]]),
            TestCase(3, 5, vec![vec![1, 2, 3], vec![1, 2, 4], vec![1, 2, 5], vec![1, 3, 4], vec![1, 3, 5], vec![1, 4, 5], vec![2, 3, 4], vec![2, 3, 5], vec![2, 4, 5], vec![3, 4, 5]])
        ];

        for TestCase(n, m, expected) in tests {
            assert_eq!(run(n, m), expected);
        }
    }
}
