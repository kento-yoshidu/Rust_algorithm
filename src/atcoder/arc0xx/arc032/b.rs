// https://atcoder.jp/contests/arc032/tasks/arc032_2

use std::collections::HashSet;
use library::lib::graph::union_find::UnionFind;

fn run(n: usize, _m: usize, ab: Vec<(usize, usize)>) -> usize {
    let mut uf = UnionFind::new(n + 1);

    for (a, b) in ab {
        uf.unite(a, b);
    }

    (1..=n)
        .map(|i| uf.find(i))
        .collect::<HashSet<usize>>()
        .len() - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, usize);

    #[test]
    fn arc032_b() {
        let tests = [
            TestCase(4, 2, vec![(1, 2), (1, 3)], 1),
            TestCase(6, 4, vec![(1, 2), (2, 3), (1, 3), (5, 6)], 2),
        ];

        for TestCase(n, m, ab, expected) in tests {
            assert_eq!(run(n, m, ab), expected);
        }
    }
}
