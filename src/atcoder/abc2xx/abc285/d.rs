// https://atcoder.jp/contests/abc285/tasks/abc285_d

use std::collections::HashMap;
use library::lib::graph::union_find::UnionFind;

fn run(_n: usize, st: Vec<(&str, &str)>) -> &'static str {
    let mut id_map = HashMap::new();
    let mut id = 0;

    for (s, t) in st.iter() {
        for x in [s, t] {
            if !id_map.contains_key(x) {
                id_map.insert(*x, id);
                id += 1;
            }
        }
    }

    let mut uf = UnionFind::new(id);

    for (s, t) in st {
        let s = id_map[&s];
        let t = id_map[&t];

        if !uf.unite(s, t) {
            return "No";
        }
    }

    "Yes"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(&'static str, &'static str)>, &'static str);

    #[test]
    fn abc285_d() {
        let tests = [
            TestCase(2, vec![("b", "m"), ("m", "d")], "Yes"),
            TestCase(3, vec![("a", "b"), ("b", "c"), ("c", "a")], "No"),
            TestCase(5, vec![("aaa", "bbb"), ("yyy", "zzz"), ("ccc", "ddd"), ("xxx", "yyy"), ("bbb", "ccc")], "Yes"),
        ];

        for TestCase(n, st, expected) in tests {
            assert_eq!(run(n, st), expected);
        }
    }
}
