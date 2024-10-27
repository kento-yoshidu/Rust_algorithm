// https://atcoder.jp/contests/abc377/tasks/abc377_a

use itertools::Itertools;

fn run(s: &str) -> &'static str {
    if s.chars().sorted().collect::<String>() == "ABC" {
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
            TestCase("BAC", "Yes"),
            TestCase("AAC", "No"),
            TestCase("ABC", "Yes"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
