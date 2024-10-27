// https://atcoder.jp/contests/abc009/tasks/abc009_1

fn run(n: usize) -> usize {
    (n + 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 1),
            TestCase(5, 3),
            TestCase(1, 1),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
