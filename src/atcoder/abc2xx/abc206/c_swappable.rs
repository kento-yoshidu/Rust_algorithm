// https://atcoder.jp/contests/abc206/tasks/abc206_c

use std::collections::HashMap;

pub fn run(n: usize, a: Vec<usize>) -> usize {
    let mut x = n * (n - 1) / 2;

    let mut hash_map = HashMap::new();

    for num in a {
        *hash_map.entry(num).or_insert(0) += 1;
    }

    for (_, v) in hash_map.iter() {
        if *v > 1 {
            x -= v*(v-1)/2;
        }
    }

    x
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![1, 7, 1], 2),
            TestCase(10, vec![1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000], 45),
            TestCase(20, vec![7, 8, 1, 1, 4, 9, 9, 6, 8, 2, 4, 1, 1, 9, 5, 5, 5, 3, 6, 4], 173),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
