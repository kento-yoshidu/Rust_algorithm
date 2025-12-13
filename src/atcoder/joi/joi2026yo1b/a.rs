// https://atcoder.jp/contests/joi2026yo1b/tasks/joi2026_yo1b_a

fn run(a: usize, b: usize) -> usize {
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn joi2026yo1b_a() {
        let tests = [
            TestCase(5, 9, 45),
            TestCase(1, 25, 25),
            TestCase(100, 100, 10000),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
