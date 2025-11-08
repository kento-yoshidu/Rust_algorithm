// https://atcoder.jp/contests/abc198/tasks/abc198_a

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
            TestCase(2, 1),
            TestCase(1, 0),
            TestCase(3, 2),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
