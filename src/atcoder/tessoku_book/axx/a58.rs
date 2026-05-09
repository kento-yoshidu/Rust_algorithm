// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_bf

use library::lib::graph::segment_tree::SegTree;

fn run(n: usize, _q: usize, query: Vec<(usize, usize, usize)>) -> Vec<i64> {
    let mut st = SegTree::new(n, 0_i64, |l, r| l.max(r));

    query.into_iter()
        .filter_map(|(q, a, b)| {
            match q {
                1 => {
                    st.set(a-1, b as i64);
                    None
                },
                2 => {
                    Some(st.prod(a-1, b-1))
                },
                _ => unreachable!(),

            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize, usize)>, Vec<i64>);

    #[test]
    fn tessoku_a58() {
        let tests = [
            TestCase(8, 4, vec![(1, 3, 16), (2, 4, 7), (1, 5, 13), (2, 4, 7)], vec![0, 13]),
        ];

        for TestCase(n, q, query, expected) in tests {
            assert_eq!(run(n, q, query), expected);
        }
    }
}
