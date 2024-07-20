// https://atcoder.jp/contests/abc158/tasks/abc158_a

use itertools::Itertools;

fn run(s: &str) -> &'static str {
    let chars: Vec<char> = s.chars().collect();

    if chars[0] == chars[1] && chars[1] == chars[2] {
        "No"
    } else {
        "Yes"
    }
}

fn run2(s: &str) -> &'static str {
    if s.chars()
        .all_equal() {
            "No"
        } else {
            "Yes"
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("ABA", "Yes"),
            TestCase("BBA", "Yes"),
            TestCase("BBB", "No"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
            assert_eq!(run2(s), expected);
        }
    }
}
