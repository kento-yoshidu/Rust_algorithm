// https://atcoder.jp/contests/abc087/tasks/arc090_b

use library::lib::graph::weighted_union_find::WeightedUnionFind;

fn run(n: usize, _m: usize, lrd: Option<Vec<(usize, usize, isize)>>) -> &'static str {
    if lrd.is_none() {
        return "Yes";
    }

    let mut wuf = WeightedUnionFind::new(n + 1);

    for (l, r, d) in lrd.unwrap() {
        if !wuf.is_valid(l, r, d) {
            return "No";
        }

        wuf.unite(l, r, d);
    }

    "Yes"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Option<Vec<(usize, usize, isize)>>, &'static str);

    #[test]
    fn arc090_d() {
        let tests = [
            TestCase(3, 3, Some(vec![(1, 2, 1), (2, 3, 1), (1, 3, 2)]), "Yes"),
            TestCase(3, 3, Some(vec![(1, 2, 1), (2, 3, 1), (1, 3, 5)]), "No"),
            TestCase(4, 3, Some(vec![(2, 1, 1), (2, 3, 5), (3, 4, 2)]), "Yes"),
            TestCase(10, 3, Some(vec![ (8, 7, 100), (7, 9, 100), (9, 8, 100)]), "No"),
            TestCase(100, 0, None, "Yes"),
        ];

        for TestCase(n, m, lrd, expected) in tests {
            assert_eq!(run(n, m, lrd), expected);
        }
    }
}

