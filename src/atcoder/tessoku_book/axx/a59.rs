// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_bg

use library::lib::graph::segment_tree::SegTree;

fn run(n: usize, _q: usize, query: Vec<(usize, usize, usize)>) -> Vec<isize> {
    let mut st = SegTree::new(n, 0_isize, |l, r| l + r);

    query.into_iter()
        .filter_map(|(q, a, b)| {
            match q {
                1 => {
                    st.set(a-1, b as isize);
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

    struct TestCase(usize, usize, Vec<(usize, usize, usize)>, Vec<isize>);

    #[test]
    fn tessoku_a59() {
        let tests = [
            TestCase(8, 4, vec![(1, 3, 16), (1, 6, 24), (2, 4, 8), (2, 1, 7)], vec![24, 40]),
        ];

        for TestCase(n, q, query, expected) in tests {
            assert_eq!(run(n, q, query), expected);
        }
    }
}
