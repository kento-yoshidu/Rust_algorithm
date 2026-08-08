// https://atcoder.jp/contests/abc028/tasks/abc028_c

use itertools::Itertools;

fn run(a: Vec<usize>) -> usize {
    let mut result: Vec<usize> = a.into_iter()
        .permutations(3)
        .map(|t| {
            t[0] + t[1] + t[2]
        })
        .collect();

    result.sort();
    result.dedup();
    result.reverse();

    result[2]
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(Vec<usize>, usize);

    #[test]
    fn abc028_c() {
        let tests = [
            TestCase(vec![1, 2, 3, 4, 5], 10),
            TestCase(vec![1, 2, 3, 5, 8], 14),
        ];

        for TestCase(a, expected) in tests {
            assert_eq!(run(a), expected);
        }
    }
}
