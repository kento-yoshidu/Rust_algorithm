// https://atcoder.jp/contests/abc338/tasks/abc338_a

pub fn run(s: &str) -> &'static str {
    if s.len() == 1 {
        if s.chars().nth(0).unwrap().is_uppercase() {
            return "Yes"
        } else {
            return "No"
        }
    }

    if s.chars().nth(0).unwrap().is_uppercase() && s[1..].chars().all(|c| !c.is_ascii_uppercase()) {
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
            TestCase("Capitalized", "Yes"),
            TestCase("AtCoder", "No"),
            TestCase("yes", "No"),
            TestCase("A", "Yes"),
            TestCase("a", "No"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
