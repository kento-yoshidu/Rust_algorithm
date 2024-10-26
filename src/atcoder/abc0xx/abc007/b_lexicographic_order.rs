// https://atcoder.jp/contests/abc007/tasks/abc007_2

fn run(a: &str) -> &'static str {
    if a == "a" {
        "-1"
    } else {
        "a"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("xyz", "a"),
            TestCase("c", "a"),
            TestCase("a", "-1"),
            TestCase("aaaaa", "a"),
        ];

        for TestCase(a, expected) in tests {
            assert_eq!(run(a), expected);
        }
    }
}
