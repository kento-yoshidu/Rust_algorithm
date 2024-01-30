// https://atcoder.jp/contests/abc336/tasks/abc336_a

pub fn run(n: usize) -> String {
    format!("L{}ng", "o".repeat(n))
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, "Looong"),
            TestCase(1, "Long"),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(expected, run(n));
        }
    }
}
