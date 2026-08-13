// https://atcoder.jp/contests/abc462/tasks/abc462_a

fn run(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_numeric())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn abc462_a() {
        let tests = [
            TestCase("abc462", "462"),
            TestCase("codequeen", ""),
            TestCase("31415", "31415"),
            TestCase("10plus2is12", "10212"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);

        }
    }
}
