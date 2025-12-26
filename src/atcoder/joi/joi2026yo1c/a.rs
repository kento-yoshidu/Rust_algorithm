// https://atcoder.jp/contests/joi2026yo1c/tasks/joi2026_yo1c_a

fn run(a: usize, b: usize) -> usize {
    a + b * 300
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn joi2026yo1c_a() {
        let tests = [
            TestCase(1000, 1, 1300),
            TestCase(8000, 2, 8600),
            TestCase(32000, 3, 32900),
            TestCase(1000, 10, 4000),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
