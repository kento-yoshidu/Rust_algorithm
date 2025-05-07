// https://atcoder.jp/contests/abc066/tasks/abc066_b

fn check(s: &str) -> bool {
    if s[0..s.len()/2] == s[s.len()/2..] {
        true
    } else {
        false
    }
}

fn run(s: &str) -> usize {
    (0..s.len())
        .rev()
        .skip(1)
        .step_by(2)
        .find(|i| {
            check(&s[0..*i])
        }).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase("abaababaab", 6),
            TestCase("xxxx", 2),
            TestCase("abcabcabcabc", 6),
            TestCase("akasakaakasakasakaakas", 14),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
