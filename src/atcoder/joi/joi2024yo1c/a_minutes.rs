// https://atcoder.jp/contests/joi2024yo1c/tasks/joi2024_yo1c_a

fn run(h: usize, m: usize) -> usize {
    h * 60 + m
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(8, 30, 510),
            TestCase(14, 0, 840),
            TestCase(0, 29, 29),
            TestCase(23, 59, 1439),
        ];

        for TestCase(h, m, expected) in tests {
            assert_eq!(run(h, m), expected);
        }
    }
}
