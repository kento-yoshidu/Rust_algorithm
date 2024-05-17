// https://atcoder.jp/contests/abc345/tasks/abc345_a

use regex::Regex;

fn run(s: &str) -> &'static str {
    let regex = Regex::new(r"^<+=+>$").unwrap();

    if regex.is_match(&s) {
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
    fn test() {
        let tests = [
            TestCase("<===>", "Yes"),
            TestCase("==>", "No"),
            TestCase("<>>", "No"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
