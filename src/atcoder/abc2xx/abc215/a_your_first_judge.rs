// https://atcoder.jp/contests/abc215/tasks/abc215_a

fn run(s: &str) -> &'static str {
    if s == "Hello,World!" {
        "AC"
    } else {
        "WA"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("Hello,World!", "AC"),
            TestCase("Hello,world!", "WA"),
            TestCase("Hello!World!", "WA"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
