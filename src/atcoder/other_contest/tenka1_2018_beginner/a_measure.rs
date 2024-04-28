// https://atcoder.jp/contests/tenka1-2018-beginner/tasks/tenka1_2018_a

fn run(s: &str) -> String {
    if s.len() == 2 {
        s.to_string()
    } else {
        s.chars()
            .rev()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("abc", "cba"),
            TestCase("ac", "ac"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
