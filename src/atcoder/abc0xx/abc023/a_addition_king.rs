// https://atcoder.jp/contests/abc023/tasks/abc023_a

fn run(n: usize) -> usize {
    n / 10 + n % 10
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(23, 5),
            TestCase(70, 7),
            TestCase(99, 18),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
