// https://atcoder.jp/contests/awc0028/tasks/awc0028_a

fn run(_n: usize, k: usize, p: Vec<usize>) -> usize {
    p.into_iter()
        .filter(|p| k <= *p)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn awc0028_a() {
        let tests = [
            TestCase(5, 60, vec![45, 72, 88, 55, 60], 220),
            TestCase(10, 50, vec![30, 85, 42, 91, 50, 67, 25, 100, 48, 73], 466),
            TestCase(15, 70, vec![65, 78, 92, 45, 70, 83, 69, 100, 55, 71, 88, 40, 95, 62, 77], 754),
        ];

        for TestCase(n, k, p, expected) in tests {
            assert_eq!(run(n, k, p), expected);
        }
    }
}
