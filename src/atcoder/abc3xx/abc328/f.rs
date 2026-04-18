// https://atcoder.jp/contests/abc328/tasks/abc328_f

use library::lib::graph::weighted_union_find::WeightedUnionFind;

fn run(n: usize, _q: usize, abd: Vec<(usize, usize, isize)>) -> Vec<usize> {
    let mut wuf = WeightedUnionFind::new(n + 1);

    abd.into_iter()
        .enumerate()
        .filter_map(|(i, (a, b, d))| {
            if !wuf.is_valid(a, b, d) {
                None
            } else {
                wuf.unite(a, b, d);
                Some(i + 1)
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize, isize)>, Vec<usize>);

    #[test]
    fn abc328_f() {
        let tests = [
            TestCase(3, 5, vec![ (1, 2, 2), (3, 2, -3), (2, 1, -1), (3, 3, 0), (1, 3, 5)], vec![1, 2, 4, 5]),
            TestCase(200000, 1, vec![(1, 1, 1)], vec![]),
            TestCase(5, 20, vec![ (4, 2, 125421359), (2, 5, -191096267), (3, 4, -42422908), (3, 5, -180492387), (3, 3, 174861038), (2, 3, -82998451), (3, 4, -134761089), (3, 1, -57159320), (5, 2, 191096267), (2, 4, -120557647), (4, 2, 125421359), (2, 3, 142216401), (4, 5, -96172984), (3, 5, -108097816), (1, 5, -50938496), (1, 2, 140157771), (5, 4, 65674908), (4, 3, 35196193), (4, 4, 0), (3, 4, 188711840)], vec![1, 2, 3, 6, 8, 9, 11, 14, 15, 16, 17, 19]),
        ];

        for TestCase(n, q, abd, expected) in tests {
            assert_eq!(run(n, q, abd), expected);
        }
    }
}
