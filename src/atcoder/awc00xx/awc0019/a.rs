// https://atcoder.jp/contests/awc0019/tasks/awc0019_a

fn run(_n: usize, k: usize, s: Vec<usize>) -> usize {
    s.into_iter()
        .filter(|s| *s >= k)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn awc0019_a() {
        let tests = [
            TestCase(5, 60, vec![72, 45, 60, 88, 59], 3),
            TestCase(10, 50, vec![100, 49, 50, 0, 75, 50, 33, 82, 51, 48], 6),
            TestCase(20, 70, vec![65, 70, 82, 55, 91, 70, 43, 88, 69, 71, 100, 0, 45, 78, 62, 95, 70, 68, 84, 50], 11),
        ];

        for TestCase(n, k, s, expected) in tests {
            assert_eq!(run(n, k, s), expected);
        }
    }
}
