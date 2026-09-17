// https://atcoder.jp/contests/awc0003/tasks/awc0003_a

fn run(_n: usize, k: usize, ab: Vec<(usize, usize)>) -> usize {
    ab.into_iter()
        .filter(|(a, b)| a * b >= k)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, usize);

    #[test]
    fn awc0003_a() {
        let tests = [
            TestCase(3, 100, vec![(10, 5), (20, 10), (5, 30)], 2),
            TestCase(5, 500, vec![(15, 40), (8, 100), (50, 20), (30, 15), (25, 25)], 4),
        ];

        for TestCase(n, k, ab, expected) in tests {
            assert_eq!(run(n, k, ab), expected);
        }
    }
}
