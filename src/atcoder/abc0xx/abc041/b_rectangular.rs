// https://atcoder.jp/contests/abc041/tasks/abc041_b

fn run(a: usize, b: usize, c: usize) -> usize {
    (a * b) % 1000000007 * c % 1000000007
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 3, 4, 24),
            TestCase(10000, 1000, 100, 1000000000),
            TestCase(100000, 1, 100000, 999999937),
            TestCase(1000000000, 1000000000, 1000000000, 999999664),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
