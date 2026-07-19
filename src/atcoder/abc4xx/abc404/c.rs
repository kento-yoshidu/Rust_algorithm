// https://atcoder.jp/contests/abc404/tasks/abc404_c

use std::collections::HashMap;

use library::lib::graph::union_find::UnionFind;

fn run(n: usize, _m: usize, ab: Vec<(usize, usize)>) -> &'static str {
    let mut map = HashMap::new();

    let mut uf = UnionFind::new(n + 1);

    for (a, b) in ab {
        map.entry(a).or_insert_with(Vec::new).push(b);
        map.entry(b).or_insert_with(Vec::new).push(a);

        uf.unite(a, b);
    }

    if map.into_iter()
        .any(|(_k, v)| {
            v.len() != 2
        }) {
            return "No";
        }

    if uf.count_roots(n) == 1 {
        "Yes"
    } else{
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, &'static str);

    #[test]
    fn abc404_c() {
        let tests = [
            TestCase(4, 4, vec![(2, 4), (3, 1), (4, 1), (2, 3)], "Yes"),
            TestCase(4, 6, vec![(1, 2), (1, 3), (1, 4), (2, 3), (2, 4), (3, 4)], "No"),
        ];

        for TestCase(n, m, ab, expected) in tests {
            assert_eq!(run(n, m, ab), expected);
        }
    }
}
