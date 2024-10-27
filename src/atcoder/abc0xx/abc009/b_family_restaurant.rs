// https://atcoder.jp/contests/abc009/tasks/abc009_2

use itertools::Itertools;

fn run(_n: usize, a: Vec<usize>) -> usize {
    let vec: Vec<usize> = a.into_iter().sorted().rev().dedup().collect();

    vec.into_iter().nth(1).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, vec![100, 200, 300, 300], 200),
            TestCase(5, vec![50, 370, 819, 433, 120], 433),
            TestCase(6, vec![100, 100, 100, 200, 200, 200], 100),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
