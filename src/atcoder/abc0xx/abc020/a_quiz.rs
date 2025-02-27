// https://atcoder.jp/contests/abc020/tasks/abc020_a

fn run(q: usize) -> &'static str {
    if q == 1 {
        "ABC"
    } else {
        "chokudai"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(1, "ABC"),
            TestCase(2, "chokudai"),
        ];

        for TestCase(q, expected) in tests {
            assert_eq!(run(q), expected);
        }
    }
}
