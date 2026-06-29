// https://atcoder.jp/contests/abc218/tasks/abc218_e

use library::lib::graph::union_find::UnionFind;

fn run(n: usize, _m: usize, abc: Vec<(usize, usize, isize)>) -> isize {
    let (neg, mut non_neg): (Vec<_>, Vec<_>) =
        abc.into_iter()
            .partition(|&(_, _, c)| c < 0);

    let mut uf = UnionFind::new(n + 1);

    for (a, b, _) in neg {
        uf.unite(a, b);
    }

    non_neg.sort_by(|a, b| a.2.cmp(&b.2));

    let mut ans = 0;

    for (a, b, c) in non_neg {
        if !uf.unite(a, b) {
            ans += c;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize, isize)>, isize);

    #[test]
    fn abc218_e() {
        let tests = [
            TestCase(4, 5, vec![(1, 2, 1), (1, 3, 1), (1, 4, 1), (3, 2, 2), (4, 2, 2)], 4),
            TestCase(3, 3, vec![(1, 2, 1), (2, 3, 0), (3, 1, -1)], 1),
            TestCase(2, 3, vec![ (1, 2, -1), (1, 2, 2), (1, 1, 3)], 5),
        ];

        for TestCase(n, m, abc, expected) in tests {
            assert_eq!(run(n, m, abc), expected);
        }
    }
}