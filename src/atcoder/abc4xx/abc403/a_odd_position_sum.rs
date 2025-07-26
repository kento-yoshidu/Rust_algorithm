// https://atcoder.jp/contests/abc403/tasks/abc403_a

fn run(_n: usize, a: Vec<usize>) -> usize {
    a.into_iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, n)| n)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(7, vec![3, 1, 4, 1, 5, 9, 2], 14),
            TestCase(1, vec![100], 100),
            TestCase(14, vec![100, 10, 1, 10, 100, 10, 1, 10, 100, 10, 1, 10, 100, 10], 403),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
