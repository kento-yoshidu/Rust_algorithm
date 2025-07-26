// https://atcoder.jp/contests/abc412/tasks/abc412_a

fn run(_n: usize, ab: Vec<(usize, usize)>) -> usize {
    ab.into_iter()
        .filter(|(a, b)| a < b)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, usize)>, usize);

    #[test]
    fn abc412_a() {
        let tests = [
            TestCase(4, vec![(2, 8), (5, 5), (5, 4), (6, 7)], 2),
            TestCase(5, vec![(1, 1), (1, 1), (1, 1), (1, 1), (1, 1)], 0),
            TestCase(6, vec![(1, 6), (2, 5), (3, 4), (4, 3), (5, 2), (6, 1)], 3),
        ];

        for TestCase(n, ab, expected) in tests {
            assert_eq!(run(n, ab), expected);
        }
    }
}
