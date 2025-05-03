// https://atcoder.jp/contests/joi2025yo1a/tasks/joi2025_yo1a_a

fn run(a: usize) -> usize {
    a / 5
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(9, 1),
            TestCase(10, 2),
            TestCase(3, 0),
            TestCase(100, 20),
        ];

        for TestCase(a, expected) in tests {
            assert_eq!(run(a), expected);
        }
    }
}
