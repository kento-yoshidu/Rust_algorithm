// https://atcoder.jp/contests/abc006/tasks/abc006_1

fn run(n: usize) -> &'static str {
    if n % 3 == 0 {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn test () {
        let tests = [
            TestCase(2, "No"),
            TestCase(9, "Yes"),
            TestCase(3, "Yes"),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
