// https://atcoder.jp/contests/abc142/tasks/abc142_b

fn run(_n: usize, k: usize, h: Vec<usize>) -> usize {
    h.into_iter()
        .filter(|heihgt| *heihgt >= k)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn abc142_b() {
        let tests = [
            TestCase(4, 150, vec![150, 140, 100, 200], 2),
            TestCase(1, 500, vec![499], 0),
            TestCase(5, 1, vec![100, 200, 300, 400, 500], 5),
        ];

        for TestCase(n, k, h, expected) in tests {
            assert_eq!(run(n, k, h), expected);
        }
    }
}
