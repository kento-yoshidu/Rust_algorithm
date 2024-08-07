// https://atcoder.jp/contests/abc285/tasks/abc285_a

fn run(a: usize, b: usize) -> &'static str {
    if a*2 == b || a*2+1 == b {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(1, 2, "Yes"),
            TestCase(2, 8, "No"),
            TestCase(14, 15, "No"),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
