// https://atcoder.jp/contests/arc035/tasks/arc035_a

fn run(s: &str) -> &'static str {
    let len = s.len();

    let chars: Vec<char> = s.chars().collect();

    for i in 0..len/2 {
        if chars[i] != '*' && chars[len-i-1] != '*' && chars[i] != chars[len-i-1] {
            return "NO";
        }
    }

    "YES"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("ab*", "YES"),
            TestCase("abc", "NO"),
            TestCase("a*bc*", "YES"),
            TestCase("***", "YES"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
