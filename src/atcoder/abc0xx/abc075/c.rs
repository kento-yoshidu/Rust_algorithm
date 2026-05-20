// https://atcoder.jp/contests/abc075/tasks/abc075_c

use library::lib::graph::union_find::UnionFind;

fn run(n: usize, m: usize, ab: Vec<(usize, usize)>) -> usize {
    let mut ans = 0;

    for i in 0..m {
        let mut uf = UnionFind::new(n + 1);

        for j in 0..m {
            if i == j {
                continue;
            }

            uf.unite(ab[j].0, ab[j].1);
        }

        if uf.count_roots(n) > 1 {
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
    fn abc075_c() {
        let tests = [
            TestCase(7, 7, vec![(1, 3), (2, 7), (3, 4), (4, 5), (4, 6), (5, 6), (6, 7)], 4),
            TestCase(3, 3, vec![(1, 2), (1, 3), (2, 3)], 0),
            TestCase(6, 5, vec![(1, 2), (2, 3), (3, 4), (4, 5), (5, 6)], 5),
        ];

        for TestCase(n, m, ab, expected) in tests {
            assert_eq!(run(n, m, ab), expected);
        }
    }
}
