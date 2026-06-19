// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_bn

use library::lib::graph::union_find::UnionFind;

fn run(n: usize, _q: usize, query: Vec<(usize, usize, usize)>) -> Vec<&'static str> {
    let mut uf = UnionFind::new(n + 1);

    query.into_iter()
        .filter_map(|(q, a, b)| {
            match q {
                1 => {
                    uf.unite(a, b);
                    None
                },
                2 => {
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
    fn tessoku_a66() {
        let tests = [
            TestCase(3, 4, vec![(1, 1, 2), (2, 1, 3), (1, 2, 3), (2, 2, 3)], vec!["No", "Yes"]),
            TestCase(12, 12, vec![ (2, 9, 11),           (1, 1, 7), (1, 1, 4), (2, 3, 6), (1, 3, 5), (2, 3, 5), (1, 10, 12), (1, 4, 8), (1, 8, 11), (2, 10, 12), (1, 5, 9), (2, 6, 8)], vec!["No", "No", "Yes", "Yes", "No"]),
        ];

        for TestCase(n, q, query, expected) in tests {
            assert_eq!(run(n, q, query), expected);
        }
    }
}
