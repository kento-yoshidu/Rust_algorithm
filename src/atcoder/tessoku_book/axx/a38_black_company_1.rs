// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_al

use std::cmp::min;

fn run(d: usize, _n: usize, lrd: Vec<(usize, usize, usize)>) -> usize {
    let mut vec = vec![24; d];

    for (l, r, d) in lrd {
        for i in l..=r {
            vec[i-1] = min(vec[i-1], d);
        }
    }

    vec.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize, usize)>, usize);

    #[test]
    fn tessoku_a38() {
        let tests = [
            TestCase(5, 3, vec![(1, 2, 22), (2, 3, 16), (3, 5, 23)], 100),
        ];

        for TestCase(d, n, lrd, expected) in tests {
            assert_eq!(run(d, n, lrd), expected);
        }
    }
}
