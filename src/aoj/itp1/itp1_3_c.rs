// https://onlinejudge.u-aizu.ac.jp/courses/lesson/2/ITP1/3/ITP1_3_C

use std::cmp::{min, max};

fn run(xy: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    xy.into_iter()
        .filter_map(|(x, y)| {
            if x == 0 && y == 0 {
                None
            } else {
                Some((min(x, y), max(x, y)))
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(Vec<(usize, usize)>, Vec<(usize, usize)>);

    #[test]
    fn itp1_3_c() {
        let tests = [
            TestCase(vec![(3, 2), (2, 2), (5, 3), (0, 0)], vec![(2, 3), (2, 2), (3, 5)]),
        ];

        for TestCase(xy, expected) in tests {
            assert_eq!(run(xy), expected);
        }
    }
}
