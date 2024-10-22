// https://atcoder.jp/contests/abc015/tasks/abc015_1

fn run<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("isuruu", "isleapyear", "isleapyear"),
            TestCase("ttttiiiimmmmeeee", "time", "ttttiiiimmmmeeee"),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
