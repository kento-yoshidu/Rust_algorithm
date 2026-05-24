// https://atcoder.jp/contests/abc325/tasks/abc325_a

fn run(s: &str, _: &str) -> String {
    format!("{} san", s)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str, &'static str);

    #[test]
    fn abc325_a() {
        let tests = [
            TestCase("Takahashi", "Chokudai", "Takahashi san"),
            TestCase("K", "Eyence", "K san"),
        ];

        for TestCase(s, t, expected) in tests {
            assert_eq!(run(s, t), expected);
        }
    }
}
