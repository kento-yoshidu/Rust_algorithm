// https://atcoder.jp/contests/typical-algorithm/tasks/typical_algorithm_f

use library::lib::graph::union_find::UnionFind;

fn run(n: usize, _m: usize, uvc: Vec<(usize, usize, usize)>) -> usize {
    let mut uvc = uvc.clone();

    uvc.sort_by(|a, b| a.2.cmp(&b.2));

    let mut uf = UnionFind::new(n + 1);

    uvc.into_iter()
        .filter_map(|(u, v, c)| {
            if uf.unite(u, v) {
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
    fn typical90_f() {
        let tests = [
            TestCase(5, 7, vec![(0, 1, 10), (0, 4, 30), (1, 2, 10), (1, 4, 20), (2, 3, 30), (4, 2, 20), (4, 3, 10)], 50),
        ];

        for TestCase(n, m, uvc, expected) in tests {
            assert_eq!(run(n, m, uvc), expected);
        }
    }
}
