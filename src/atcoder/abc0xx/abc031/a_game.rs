// https://atcoder.jp/contests/abc031/tasks/abc031_a

fn run(a: usize, d: usize) -> usize {
    a.max(d) * (a.min(d) + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(31, 87, 2784),
            TestCase(101, 65, 6666),
            TestCase(3, 3, 12),
        ];

        for TestCase(n, d, expected) in tests {
            assert_eq!(run(n, d), expected);
        }
    }
}
