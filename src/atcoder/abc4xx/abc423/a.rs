// https://atcoder.jp/contests/abc423/tasks/abc423_a

fn run(x: usize, c: usize) -> usize {
    x / (1000 + c) * 1000
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn abc423_a() {
        let tests = [
            TestCase(650_000, 8, 644_000),
            TestCase(1003, 4, 0),
            TestCase(10_000_000, 24, 9_765_000),
        ];

        for TestCase(x, c, expected) in tests {
            assert_eq!(run(x, c), expected);
        }
    }
}
