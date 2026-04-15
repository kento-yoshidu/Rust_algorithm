// https://atcoder.jp/contests/abc304/tasks/abc304_e

use std::collections::HashSet;
use std::cmp::{min, max};
use library::lib::graph::union_find::UnionFind;

fn run(
    n: usize,
    _m: usize,
    uv: Vec<(usize, usize)>,
    _k: usize,
    xy: Vec<(usize, usize)>,
    _q: usize,
    pq: Vec<(usize, usize)>
) -> Vec<&'static str> {
    let mut uf = UnionFind::new(n + 1);

    for (u, v) in uv {
        uf.unite(u, v);
    }

    let mut ng_pair = HashSet::new();

    for (x, y) in xy {
        let x = uf.find(x);
        let y = uf.find(y);

        ng_pair.insert((min(x, y), max(x, y)));
    }

    pq
        .into_iter()
        .map(|(p, q)| {
            let p = uf.find(p);
            let q = uf.find(q);

            if ng_pair.contains(&(min(p, q), max(p, q))) {
                "No"
            } else {
                "Yes"
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, usize, Vec<(usize, usize)>, usize, Vec<(usize, usize)>, Vec<&'static str>);

    #[test]
    fn abc304_e() {
        let tests = [
            TestCase(6, 6, vec![(1, 2), (2, 3), (2, 3), (3, 1), (5, 4), (5, 5)], 3, vec![(1, 5), (2, 6), (4, 3)], 4, vec![ (2, 5), (2, 6), (5, 6), (5, 4)], vec!["No", "No", "Yes", "Yes"]),
        ];

        for TestCase(n, m, uv, k, xy, q, pq, expected) in tests {
            assert_eq!(run(n, m, uv, k, xy, q, pq), expected);
        }
    }
}
