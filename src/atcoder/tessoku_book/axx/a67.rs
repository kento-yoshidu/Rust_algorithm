// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_bo

use itertools::Itertools;
use library::lib::graph::union_find::UnionFind;

fn run(n: usize, _m: usize, abc: Vec<(usize, usize, usize)>) -> usize {
    let abc: Vec<(usize, usize, usize)> = abc.into_iter().sorted_by(|a, b| a.2.cmp(&b.2)).collect();

    let mut uf = UnionFind::new(n + 1);

    abc.into_iter()
        .filter_map(|(a, b, c)| {
            if uf.unite(a, b) {
                Some(c)
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize, usize)>, usize);

    #[test]
    fn tessoku_a67() {
        let tests = [
            TestCase(7, 9, vec![ (1, 2, 12), (1, 3, 10), (2, 6, 160), (2, 7, 15), (3, 4, 1), (3, 5, 4), (4, 5, 3), (4, 6, 120), (6, 7, 14)], 55),
        ];

        for TestCase(n, m, abc, expected) in tests {
            assert_eq!(run(n, m, abc), expected);
        }
    }
}
