// https://atcoder.jp/contests/abc405/tasks/abc405_b

use std::collections::HashSet;

fn run(n: usize, m: usize, a: Vec<usize>) -> usize {
    let a = a.clone();

    for i in 0..=n {
        let vec = &a[..n-i];
        let hash_set: Vec<usize> = vec.iter().cloned().collect();

        if hash_set.len() < m || (1..=m).any(|x| !hash_set.contains(&x)) {
            return i;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn abc405_b() {
        let tests = [
            TestCase(5, 3, vec![3, 2, 3, 1, 2], 2),
            TestCase(4, 3, vec![1, 3, 1, 3], 0),
            TestCase(10, 4, vec![1, 3, 3, 4, 2, 1, 4, 1, 2, 4], 6),
        ];

        for TestCase(n, m, a, expected) in tests {
            assert_eq!(run(n, m, a), expected);
        }
    }
}
