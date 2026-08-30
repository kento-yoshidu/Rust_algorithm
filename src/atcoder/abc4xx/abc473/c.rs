// https://atcoder.jp/contests/abc473/tasks/abc473_c

use std::collections::HashMap;

fn run(_n: usize, _k: usize, a: Vec<usize>) -> usize {
    let mut map = HashMap::new();

    for a in a {
        *map.entry(a).or_insert(0) += 1;
    }

    let max = map.iter()
        .map(|(_, v)| v)
        .max()
        .unwrap();

    map.iter()
        .filter(|(_, v)| {
            v == &max || (*v + 1 == *max)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn abc473_c() {
        let tests = [
            TestCase(8, 5, vec![3, 3, 5, 5, 4, 4, 3, 2], 3),
            TestCase(6, 1, vec![1, 1, 1, 1, 1, 1], 1),
            TestCase(14, 8, vec![6, 1, 5, 3, 8, 4, 3, 4, 3, 5, 1, 2, 5, 1], 4),
        ];

        for TestCase(n, k, a, expected) in tests {
            assert_eq!(run(n, k, a), expected);
        }
    }
}
