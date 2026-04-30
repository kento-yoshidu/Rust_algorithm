// https://atcoder.jp/contests/abc120/tasks/abc120_d

use library::lib::graph::union_find::UnionFind;

fn run(n: usize, _m: usize, ab: Vec<(usize, usize)>) -> Vec<usize> {
    let mut uf = UnionFind::new(n + 1);
    let mut cur = n * (n - 1) / 2;

    let mut ans = Vec::new();

    for (a, b) in ab.into_iter().rev() {
        ans.push(cur);

        if !uf.same(a, b) {
            cur -= uf.size(a) * uf.size(b);

            uf.unite(a, b);
        }
    }

    ans.into_iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, Vec<usize>);

    #[test]
    fn abc120_d() {
        let tests = [
            TestCase(4, 5, vec![(1, 2), (3, 4), (1, 3), (2, 3), (1, 4)], vec![0, 0, 4, 5, 6]),
            TestCase(6, 5, vec![(2, 3), (1, 2), (5, 6), (3, 4), (4, 5)], vec![8, 9, 12, 14, 15]),
            TestCase(2, 1, vec![(1, 2)], vec![1]),
        ];

        for TestCase(n, m, ab, expected) in tests {
            assert_eq!(run(n, m, ab), expected);
        }
    }
}
