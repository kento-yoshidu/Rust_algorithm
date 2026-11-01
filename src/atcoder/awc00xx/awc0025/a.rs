// https://atcoder.jp/contests/awc0025/tasks/awc0025_a

fn run(_n: usize, x: usize, a: Vec<usize>) -> usize {
    a.into_iter()
        .filter(|a| *a < x)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn awc0025_a() {
        let tests = [
            TestCase(5, 60, vec![72, 45, 60, 38, 85], 2),
            TestCase(8, 50, vec![49, 50, 51, 30, 75, 50, 48, 100], 3),
            TestCase(15, 70, vec![65, 70, 82, 55, 91, 68, 70, 43, 77, 69, 100, 35, 88, 71, 60], 7),
        ];

        for TestCase(n, x, a, expected) in tests {
            assert_eq!(run(n, x, a), expected);
        }
    }
}
