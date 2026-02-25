// https://yukicoder.me/problems/no/69

use itertools::Itertools;

fn run(a: &str, b: &str) -> &'static str {
    if a.chars().sorted().collect::<String>() == b.chars().sorted().collect::<String>() {
        "YES"
    } else {
        "NO"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str, &'static str);

    #[test]
    fn yuki_069() {
        let tests = [
            TestCase("dog", "god", "YES"),
            TestCase("cat", "tea", "NO"),
            TestCase("silence", "license", "YES"),
            TestCase("yukicoder", "yukicoder", "YES"),
            TestCase("hurjztkyua", "urjukzthua", "NO"),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
