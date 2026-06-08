// https://atcoder.jp/contests/abc231/tasks/abc231_d

use library::lib::graph::union_find::UnionFind;

fn run(n: usize, _m: usize, ab: Vec<(usize, usize)>) -> &'static str {
    let mut count = vec![0; n + 1];
    let mut uf = UnionFind::new(n + 1);

    for (a, b) in ab {
        count[a] += 1;
        count[b] += 1;

        if count[a] > 2 || count[b] > 2 {
            return "No";
        }

        if !uf.unite(a, b) {
            return "No";
        }
    }

    "Yes"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, &'static str);

    #[test]
    fn abc231_d() {
        let tests = [
            TestCase(4, 2, vec![(1, 3), (2, 3)], "Yes"),
            TestCase(4, 3, vec![(1, 4), (2, 4), (3, 4)], "No"),
        ];

        for TestCase(n, m, ab, expected) in tests {
            assert_eq!(run(n, m, ab), expected);
        }
    }
}
