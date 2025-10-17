// https://yukicoder.me/problems/no/730

use std::collections::HashSet;

fn run(s: &str) -> &'static str {
    let set: HashSet<char> = s.chars().collect();

    if set.len() == s.len() {
        "YES"
    } else {
        "NO"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn yuki_730() {
        let tests = [
            TestCase("YUKICODER", "YES"),
            TestCase("AAA", "NO"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
