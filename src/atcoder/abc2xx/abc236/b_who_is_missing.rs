// https://atcoder.jp/contests/abc236/tasks/abc236_b

use std::collections::HashMap;

fn run(_n: usize, a: Vec<usize>) -> usize {
    let mut hash_map = HashMap::new();

    for i in a {
        *hash_map.entry(i).or_insert(0) += 1;
    }

    hash_map.into_iter()
        .find(|(_, count)| {
            *count == 3
        })
        .unwrap()
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn abc236_b() {
        let tests = [
            TestCase(3, vec![1, 3, 2, 3, 3, 2, 2, 1, 1, 1, 2], 3),
            TestCase(1, vec![1, 1, 1], 1),
            TestCase(4, vec![3, 2, 1, 1, 2, 4, 4, 4, 4, 3, 1, 3, 2, 1, 3], 2),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
