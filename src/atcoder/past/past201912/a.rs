// https://atcoder.jp/contests/past201912-open/tasks/past201912_a

fn run(s: &str) -> String {
    if s.chars().any(|c| !c.is_digit(10)) {
        String::from("error")
    } else {
        let num: usize = s.parse().unwrap();

        (num * 2).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn past201912_a() {
        let tests = [
            TestCase("678", "1356"),
            TestCase("abc", "error"),
            TestCase("0x8", "error"),
            TestCase("012", "24"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
