// https://atcoder.jp/contests/arc056/tasks/arc056_b

use std::collections::HashMap;
use library::lib::graph::union_find::UnionFind;

fn run(n: usize, _m: usize, s: usize, uv: Vec<(usize, usize)>) -> Vec<usize> {
    let mut map = HashMap::new();

    for (u, v) in uv {
        map.entry(u).or_insert_with(|| Vec::new()).push(v);
        map.entry(v).or_insert_with(|| Vec::new()).push(u);
    }

    let mut used = vec![false; n + 1];

    let mut uf = UnionFind::new(n + 1);
    let mut ans = Vec::new();

    for i in (1..=n).rev() {
        used[i] = true;

        if let Some(next) = map.get(&i) {
            for n in next {
                if used[*n] {
                    uf.unite(i, *n);
                }
            }

            if uf.same(i, s) {
                ans.push(i);
            }
        }
    }

    ans.into_iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<(usize, usize)>, Vec<usize>);

    #[test]
    fn arc056_b() {
        let tests = [
            TestCase(3, 3, 2, vec![(1, 2), (2, 3), (1, 3)], vec![1, 2]),
            TestCase(5, 6, 5, vec![(1, 5), (3, 5), (3, 2), (4, 1), (1, 2), (4, 3)], vec![1, 2, 3, 5]),
            TestCase(5, 5, 5, vec![(1, 4), (4, 3), (3, 2), (2, 5), (5, 1)], vec![1, 2, 5]),
        ];

        for TestCase(n, m, s, uv, expected) in tests {
            assert_eq!(run(n, m, s, uv), expected);
        }
    }
}
