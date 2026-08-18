// https://atcoder.jp/contests/abc126/tasks/abc126_e

use library::lib::graph::union_find::UnionFind;

fn run(n: usize, _m: usize, xyz: Vec<(usize, usize, usize)>) -> usize {
    let mut uf = UnionFind::new(n + 1);

    for (x, y, _) in xyz {
        uf.unite(x, y);
    }

    uf.count_roots(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize, usize)>, usize);

    #[test]
    fn abc126_e() {
        let tests = [
            TestCase(3, 1, vec![(1, 2, 1)], 2),
            TestCase(6, 5, vec![(1, 2, 1), (2, 3, 2), (1, 3, 3), (4, 5, 4), (5, 6, 5)], 2),
            TestCase(100000, 1, vec![(1, 100000, 100)], 999999),
        ];

        for TestCase(n, m, xyz, expected) in tests {
            assert_eq!(run(n, m, xyz), expected);
        }
    }
}
