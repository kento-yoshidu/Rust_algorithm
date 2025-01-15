// https://atcoder.jp/contests/abc002/tasks/abc002_1

fn run(a: usize, b: usize) -> usize {
    a.max(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(10, 11, 11),
            TestCase(100000000, 10000000, 100000000),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
