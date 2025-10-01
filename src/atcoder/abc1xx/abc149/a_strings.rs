// https://atcoder.jp/contests/abc149/tasks/abc149_a

fn run(s: &str, t: &str) -> String {
    format!("{}{}", t, s)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str, &'static str);

    #[test]
    fn abc149_a() {
        let tests = [
            TestCase("oder", "atc", "atcoder"),
            TestCase("humu", "humu", "humuhumu"),
        ];

        for TestCase(s, t, expected) in tests {
            assert_eq!(run(s, t), expected);
        }
    }
}
