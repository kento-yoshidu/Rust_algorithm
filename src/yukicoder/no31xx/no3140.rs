// https://yukicoder.me/problems/no/3140

fn run(_n: usize, _s: &str) -> &'static str {
    "First"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, &'static str);

    #[test]
    fn yuki_3140() {
        let tests = [
            TestCase(6, "()(())", "First"),
            TestCase(2, "()", "First"),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
