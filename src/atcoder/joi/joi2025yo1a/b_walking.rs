// https://atcoder.jp/contests/joi2025yo1a/tasks/joi2025_yo1a_b

fn run(x: usize) -> usize {
    if x % 2 == 0 {
        x / 2
    } else {
        x / 2 + 3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 4),
            TestCase(6, 3),
            TestCase(1, 3),
            TestCase(37, 21),
            TestCase(100, 50),
        ];

        for TestCase(x, expected) in tests {
            assert_eq!(run(x), expected);
        }
    }
}
