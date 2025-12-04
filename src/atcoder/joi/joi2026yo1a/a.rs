// https://atcoder.jp/contests/joi2026yo1a/tasks/joi2026_yo1a_a

fn run(y: usize, b: usize) -> usize {
    y * b * b
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn joi2026yo1a() {
        let tests = [
            TestCase(3, 2, 12),
            TestCase(10, 5, 250),
            TestCase(1, 1, 1),
            TestCase(100, 100, 1000000),
        ];

        for TestCase(y, b, expected) in tests {
            assert_eq!(run(y, b), expected);
        }
    }
}
