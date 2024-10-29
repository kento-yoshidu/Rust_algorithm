// https://atcoder.jp/contests/abc011/tasks/abc011_1

fn run(n: usize) -> usize {
    n % 12 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(1, 2),
            TestCase(12, 1),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
