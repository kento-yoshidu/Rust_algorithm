// https://atcoder.jp/contests/abc473/tasks/abc473_b

use std::collections::HashMap;

fn run(_n: usize, a: Vec<usize>) -> usize {
    let mut map = HashMap::new();

    for a in a {
        *map.entry(a).or_insert(0) += 1;
    }

    map.into_iter()
        .filter(|(_, v)| {
            v % 2 != 0
        })
        .map(|(k, _)| k)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn abc473_b() {
        let tests = [
            TestCase(8, vec![2, 7, 1, 8, 2, 8, 1, 8], 15),
            TestCase(5, vec![1, 2, 3, 4, 5], 15),
            TestCase(15, vec![58, 97, 74, 16, 97, 74, 97, 16, 51, 52, 58, 52, 74, 32, 43], 297),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
