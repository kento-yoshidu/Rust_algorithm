// https://atcoder.jp/contests/abc177/tasks/abc177_d

use library::lib::graph::union_find::UnionFind;

fn run(n: usize, _m: usize, ab: Vec<(usize, usize)>) -> usize {
    let mut uf = UnionFind::new(n + 1);

    for (a, b) in ab {
        uf.unite(a, b);
    }

    (1..=n)
        .map(|n| uf.size(n))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, usize);

    #[test]
    fn abc177_d() {
        let tests = [
            TestCase(5, 3, vec![(1, 2), (3, 4), (5, 1)], 3),
            TestCase(4, 10, vec![ (1, 2), (2, 1), (1, 2), (2, 1), (1, 2), (1, 3), (1, 4), (2, 3), (2, 4), (3, 4)], 4),
            TestCase(10, 4, vec![(3, 1), (4, 1), (5, 9), (2, 6)], 3),
        ];

        for TestCase(n, m, ab, expected) in tests {
            assert_eq!(run(n, m, ab), expected);
        }
    }
}
