// https://atcoder.jp/contests/abc341/tasks/abc341_a

pub fn run(n: usize) -> String {
    format!("{}1", "10".repeat(n))
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, "101010101"),
            TestCase(1, "101"),
            TestCase(10, "101010101010101010101"),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected)
        }
    }
}
