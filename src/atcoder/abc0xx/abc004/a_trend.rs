// https://atcoder.jp/contests/abc004/tasks/abc004_1

fn run(n: usize) -> usize {
    n * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(1000, 2000),
            TestCase(1000000, 2000000),
            TestCase(0, 0),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
