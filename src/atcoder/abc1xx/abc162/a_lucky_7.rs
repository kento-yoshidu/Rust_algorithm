// https://atcoder.jp/contests/abc162/tasks/abc162_a

fn run(s: &'static str) -> &'static str {
    if s.chars()
        .any(|c| {
            c == '7'
        }) {
            "Yes"
        } else {
            "No"
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn abc162_a() {
        let tests = [
            TestCase("117", "Yes"),
            TestCase("123", "No"),
            TestCase("777", "Yes"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
