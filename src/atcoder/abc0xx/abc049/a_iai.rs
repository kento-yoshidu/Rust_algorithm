// https://atcoder.jp/contests/abc049/tasks/abc049_a

fn run(c: &str) -> &'static str {
    match c {
        "a" | "i" | "u" | "e" | "o" => "vowel",
        _ => "consonant"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("a", "vowel"),
            TestCase("z", "consonant"),
            TestCase("s", "consonant"),
        ];

        for TestCase(c, expected) in tests {
            assert_eq!(run(c), expected);
        }
    }
}
