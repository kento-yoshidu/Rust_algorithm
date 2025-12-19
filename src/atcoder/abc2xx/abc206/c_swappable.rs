// https://atcoder.jp/contests/abc206/tasks/abc206_c

use std::collections::HashMap;

fn run(n: usize, a: Vec<usize>) -> usize {
    let x = n*(n-1)/2;

    let mut hash_map = HashMap::new();

    for num in a {
        *hash_map.entry(num).or_insert(0) += 1;
    }

    hash_map.into_iter()
        .fold(x, |state, (_, v)| {
            if v > 1 {
                state - v*(v-1)/2
            } else {
                state
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn abc206_c() {
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
