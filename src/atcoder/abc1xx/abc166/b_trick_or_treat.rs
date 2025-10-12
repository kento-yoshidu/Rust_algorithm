// https://atcoder.jp/contests/abc166/tasks/abc166_b

use std::collections::HashSet;

fn run(n: usize, _k: usize, vec: Vec<(usize, Vec<usize>)>) -> usize {
    let mut hash_set = HashSet::new();

    for v in vec {
        for num in v.1 {
            hash_set.insert(num);
        }
    }

    (1..=n)
        .filter(|num| {
            !hash_set.contains(num)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, Vec<usize>)>, usize);

    #[test]
    fn abc166_b() {
        let tests = [
            TestCase(3, 2, vec![(2, vec![1, 3]), (1, vec![3])], 1),
            TestCase(3, 3, vec![(1, vec![3]), (1, vec![3]), (1, vec![3])], 2),
        ];

        for TestCase(n, k, v, expected) in tests {
            assert_eq!(run(n, k, v), expected);
        }
    }
}
