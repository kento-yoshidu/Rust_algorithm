// https://atcoder.jp/contests/arc037/tasks/arc037_b

use std::collections::HashMap;
use library::lib::graph::union_find::UnionFind;

fn run(n: usize, _m: usize, uv: Vec<(usize, usize)>) -> usize {
    let mut uf = UnionFind::new(n + 1);

    for &(u, v) in &uv {
        uf.unite(u, v);
    }

    // 頂点数
    let mut v_count = HashMap::new();

    for i in 1..=n {
        let root = uf.find(i);

        *v_count.entry(root).or_insert(0) += 1;
    }

    // 辺数
    let mut e_count = HashMap::new();

    for &(u, _) in &uv {
        let root = uf.find(u);

        *e_count.entry(root).or_insert(0) += 1;
    }

    let mut ans = 0;

    for (root, &v) in &v_count {
        let e = e_count.get(root).unwrap_or(&0);

        if *e == v - 1 {
            ans += 1;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, usize);

    #[test]
    fn arc037_b() {
        let tests = [
            TestCase(8, 7, vec![ (1, 2), (2, 3), (2, 4), (5, 6), (6, 7), (6, 8), (7, 8)], 1),
            TestCase(5, 1, vec![(1, 2)], 4),
            TestCase(11, 11, vec![ (1, 2), (1, 3), (2, 4), (3, 5), (4, 6), (5, 7), (6, 8), (7, 9), (8, 10), (9, 11), (10, 11)], 0),
        ];

        for TestCase(n, m, uv, expected) in tests {
            assert_eq!(run(n, m, uv), expected);
        }
    }
}
