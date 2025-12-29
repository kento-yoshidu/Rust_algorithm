// https://atcoder.jp/contests/abc220/tasks/abc220_b

fn run(k: u32, a: &str, b: &str) -> usize {
    usize::from_str_radix(&a, k).unwrap() * usize::from_str_radix(&b, k).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(u32, &'static str, &'static str, usize);

    #[test]
    fn abc220_b() {
        let tests = [
            TestCase(2, "1011", "10100", 220),
            TestCase(7, "123", "456", 15642),
        ];

        for TestCase(k, a, b, expected) in tests {
            assert_eq!(run(k, a, b), expected);
        }
    }
}
