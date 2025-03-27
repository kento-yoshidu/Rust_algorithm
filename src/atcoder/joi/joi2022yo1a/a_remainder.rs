// https://atcoder.jp/contests/joi2022yo1a/tasks/joi2022_yo1a_a

fn run(x: usize) -> usize {
    x % 21
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(50, 8),
            TestCase(42, 0),
            TestCase(5, 5),
        ];

        for TestCase(x, expected) in tests {
            assert_eq!(run(x), expected);
        }
    }
}
