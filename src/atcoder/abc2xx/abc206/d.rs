// https://atcoder.jp/contests/abc206/tasks/abc206_d

use std::collections::{HashMap, HashSet};
use library::lib::graph::union_find::UnionFind;

fn run(n: usize, a: Vec<usize>) -> usize {
    let mut uf = UnionFind::new(200_001);

    for i in 0..n/2 {
        if a[i] != a[n - 1 - i] {
            uf.unite(a[i], a[n - 1 - i]);
        }
    }


    let mut map = HashMap::new();

    for &v in &a {
        let root = uf.find(v);
        map.entry(root).or_insert(HashSet::new()).insert(v);
    }

    let mut ans = 0;

    for (_, set) in map {
        ans += set.len() - 1;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn abc206_d() {
        let tests = [
            TestCase(8, vec![1, 5, 3, 2, 5, 2, 3, 1], 2),
            TestCase(7, vec![1, 2, 3, 4, 1, 2, 3], 1),
            TestCase(1, vec![200000], 0),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
