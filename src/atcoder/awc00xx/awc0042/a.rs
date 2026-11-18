// https://atcoder.jp/contests/awc0042/tasks/awc0042_a

fn run(_n: usize, k: usize, ab: Vec<(usize, usize)>) -> usize {
    ab.into_iter()
        .map(|(a, b)| {
            (a + b + k - 1) / k
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, usize);

    #[test]
    fn awc0042_a() {
        let tests = [
            TestCase(3, 5, vec![(1, 2), (4, 0), (3, 5)], 4),
            TestCase(4, 3, vec![(0, 0), (1, 2), (3, 0), (2, 5)], 5),
            TestCase(8, 7, vec![(2, 3), (7, 0), (6, 6), (0, 5), (14, 1), (8, 13), (0, 0), (20, 4)], 15),
            TestCase(15, 10, vec![(100, 0), (0, 1), (9, 9), (10, 10), (123, 456), (999999999, 1), (500000000, 500000000), (17, 23), (0, 0), (42, 58), (1000000000, 1000000000), (314159265, 271828182), (5, 4), (19, 0), (0, 999999999)], 558598835),
            TestCase(1, 1000000000, vec![(0, 0)], 0),
        ];

        for TestCase(n, k, ab, expected) in tests {
            assert_eq!(run(n, k, ab), expected);
        }
    }
}
