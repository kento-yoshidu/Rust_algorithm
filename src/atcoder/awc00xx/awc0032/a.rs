// https://atcoder.jp/contests/awc0032/tasks/awc0032_a

use library::lib::distance;

fn run(_n: usize, r: usize, xypq: Vec<(isize, isize, isize, isize)>) -> usize {
    xypq
        .into_iter()
        .filter(|(x, y, p, q)| {
            distance::is_within_euclidean_distance(*x, *y, *p, *q, r)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(isize, isize, isize, isize)>, usize);

    #[test]
    fn awc0032_a() {
        let tests = [
            TestCase(3, 5, vec![(0, 0, 3, 4), (0, 0, 6, 0), (1, 1, 1, 1)], 2),
            TestCase(2, 1, vec![(0, 0, 10, 10), (5, 5, 0, 0)], 0),
            TestCase(6, 10, vec![(0, 0, 3, 4), (0, 0, 8, 6), (0, 0, 11, 0), (5, 5, 5, 5), (-3, 4, 3, -4), (100, 100, 0, 0)], 4),
            TestCase(10, 1000000000, vec![(0, 0, 600000000, 800000000), (0, 0, 600000001, 800000000), (1000000000, 1000000000, -1000000000, -1000000000), (0, 0, 0, 0), (-500000000, -500000000, -500000000, -500000000), (999999999, 0, 0, 0), (0, 0, 0, 1000000000), (100, 200, 300, 400), (-1000000000, 0, 1000000000, 0), (500000000, 500000000, 500000001, 500000001)], 7),
            TestCase(1, 1, vec![(0, 0, 1, 0)], 1),
        ];

        for TestCase(n, r, xypq, expected) in tests {
            assert_eq!(run(n, r, xypq), expected);
        }
    }
}
