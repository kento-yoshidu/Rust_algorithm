// https://yukicoder.me/problems/no/163

fn run(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_ascii_lowercase()
            } else {
                c.to_ascii_uppercase()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn yuki_163() {
        let tests = [
            TestCase("CapsLock", "cAPSlOCK"),
            TestCase("ABCdef", "abcDEF"),
            TestCase("A", "a"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
