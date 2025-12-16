// https://atcoder.jp/contests/abc249/tasks/abc249_b

use itertools::Itertools;

fn run(s: &str) -> &'static str {
    if !s.chars().all_unique() {
        return "No"
    }

    if s.chars().all(|c| {
        c.is_uppercase()
    }) {
        return "No"
    }

    if s.chars().all(|c| {
        c.is_lowercase()
    }) {
        return "No";
    }

    "Yes"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn abc249_b() {
        let tests = [
            TestCase("Aa", "Yes"),
            TestCase("AtCoder", "Yes"),
            TestCase("atcoder", "No"),
            TestCase("Perfect", "No"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
