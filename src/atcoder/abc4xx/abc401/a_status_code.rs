// https://atcoder.jp/contests/abc401/tasks/abc401_a

fn run(s: usize) -> &'static str {
    if 200 <= s && s <= 299 {
        "Success"
    } else {
        "Failure"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(200, "Success"),
            TestCase(401, "Failure"),
            TestCase(999, "Failure"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
