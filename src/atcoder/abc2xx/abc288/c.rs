// https://atcoder.jp/contests/abc288/tasks/abc288_c

use library::lib::graph::union_find::UnionFind;

fn run(n: usize, _m: usize, ab: Vec<(usize, usize)>) -> usize {
    let mut uf = UnionFind::new(n + 1);

    ab.into_iter()
        .filter(|(a, b)| {
            !uf.unite(*a, *b)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, usize);

    #[test]
    fn abc288_c() {
        let tests = [
            TestCase(6, 7, vec![(1, 2), (1, 3), (2, 3), (4, 2), (6, 5), (4, 6), (4, 5)], 2),
            TestCase(4, 2, vec![(1, 2), (3, 4)], 0),
            TestCase(5, 3, vec![(1, 2), (1, 3), (2, 3)], 1),
        ];

        for TestCase(n, m, ab, expected) in tests {
            assert_eq!(run(n, m, ab), expected);
        }
    }
}
