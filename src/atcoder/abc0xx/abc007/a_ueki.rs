// https://atcoder.jp/contests/abc007/tasks/abc007_1

fn run(n: usize) -> usize {
    n - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, 3),
            TestCase(100, 99),
            TestCase(1, 0),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
