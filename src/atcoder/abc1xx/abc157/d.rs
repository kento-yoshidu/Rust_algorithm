// https://atcoder.jp/contests/abc157/tasks/abc157_d

use std::collections::HashSet;
use library::lib::graph::union_find::UnionFind;

fn run(n: usize, _m: usize, _k: usize, ab: Vec<(usize, usize)>, cd: Vec<(usize, usize)>) -> Vec<usize> {
    let mut uf = UnionFind::new(n + 1);

    let mut ng = vec![HashSet::new(); n + 1];

    for (a, b) in ab {
        ng[a].insert(b);
        ng[b].insert(a);

        uf.unite(a, b);
    }

    for (c, d) in cd {
        if uf.same(c, d) {
            ng[c].insert(d);
            ng[d].insert(c);
        }
    }

    (1..=n)
        .map(|i| {
            let s = uf.size(i);
            s - 1 - ng[i].len()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>);

    #[test]
    fn abc157_d() {
        let tests = [
            TestCase(4, 4, 1, vec![(2, 1), (1, 3), (3, 2), (3, 4)], vec![(4, 1)], vec![0, 1, 0, 1]),
            TestCase(5, 10, 0, vec![(1, 2), (1, 3), (1, 4), (1, 5), (3, 2), (2, 4), (2, 5), (4, 3), (5, 3), (4, 5)], vec![], vec![0, 0, 0, 0, 0]),
            TestCase(10, 9, 3, vec![(10, 1), (6, 7), (8, 2), (2, 5), (8, 4), (7, 3), (10, 9), (6, 4), (5, 8)], vec![(2, 6), (7, 5), (3, 1)], vec![1, 3, 5, 4, 3, 3, 3, 3, 1, 0]),
        ];

        for TestCase(n, m, k, ab, cd, expected) in tests {
            assert_eq!(run(n, m, k, ab, cd), expected);
        }
    }
}
