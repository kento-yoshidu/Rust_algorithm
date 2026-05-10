// https://atcoder.jp/contests/atc001/tasks/unionfind_a

use library::lib::graph::union_find::UnionFind;

fn run(n: usize, _q: usize, pab: Vec<(usize, usize, usize)>) -> Vec<&'static str> {
    let mut uf = UnionFind::new(n + 1);

    pab
        .into_iter()
        .filter_map(|(p, a, b)| {
            match p {
                0 => {
                    uf.unite(a, b);
                    None
                },
                1 => {
                    if uf.same(a, b) {
                        Some("Yes")
                    } else {
                        Some("No")
                    }
                },
                _ => unreachable!(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize, usize)>, Vec<&'static str>);

    #[test]
    fn atc_b() {
        let tests = [
            TestCase(8, 9, vec![ (0, 1, 2), (0, 3, 2), (1, 1, 3), (1, 1, 4), (0, 2, 4), (1, 4, 1), (0, 4, 2), (0, 0, 0), (1, 0, 0)], vec!["Yes", "No", "Yes", "Yes"]),
        ];

        for TestCase(n, q, pab, expected) in tests {
            assert_eq!(run(n, q, pab), expected);
        }
    }
}
