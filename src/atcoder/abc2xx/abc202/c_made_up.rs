// https://atcoder.jp/contests/abc202/tasks/abc202_c

use std::collections::HashMap;

fn run(n: usize, a: Vec<usize>, b: Vec<usize>, c: Vec<usize>) -> usize {
    let mut hash_map = HashMap::new();

    for i in 0..n {
        *hash_map.entry(b[c[i]-1]).or_insert(0) += 1;
    }

    (0..n)
        .filter_map(|i| {
            hash_map.get(&a[i])
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<usize>, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![1, 2, 2], vec![3, 1, 2], vec![2, 3, 2], 4),
            TestCase(4, vec![1, 1, 1, 1], vec![1, 1, 1, 1], vec![1, 2, 3, 4], 16),
            TestCase(3, vec![2, 3, 3], vec![1, 3, 3], vec![1, 1, 1], 0),
        ];

        for TestCase(n, a, b, c, expected) in tests {
            assert_eq!(run(n, a, b, c), expected);
        }
    }
}
