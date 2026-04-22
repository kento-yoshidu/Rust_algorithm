// https://atcoder.jp/contests/abc333/tasks/abc333_d

use library::lib::graph::union_find::UnionFind;

fn run(n: usize, uv: Vec<(usize, usize)>) -> usize {
    let mut uf = UnionFind::new(n + 1);
    let mut max_size = 0;

    for (u, v) in uv {
        if u != 1 {
            uf.unite(u, v);
        }

        max_size = max_size.max(uf.size(u));
    }

    n - max_size
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, usize)>, usize);

    #[test]
    fn abc333_d() {
        let tests = [
            TestCase(9, vec![(1, 2), (2, 3), (2, 4), (2, 5), (1, 6), (6, 7), (7, 8), (7, 9)], 5),
            TestCase(6, vec![(1, 2), (2, 3), (2, 4), (3, 5), (3, 6)], 1),
            TestCase(24, vec![(3, 6), (7, 17), (7, 20), (7, 11), (14, 18), (17, 21), (6, 19), (5, 22), (9, 24), (11, 14), (6, 23), (8, 17), (9, 12), (4, 17), (2, 15), (1, 17), (3, 9), (10, 16), (7, 13), (2, 16), (1, 16), (5, 7), (1, 3)], 12),
        ];

        for TestCase(n, uv, expected) in tests {
            assert_eq!(run(n, uv), expected);
        }
    }
}
