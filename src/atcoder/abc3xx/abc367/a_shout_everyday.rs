// https://atcoder.jp/contests/abc367/tasks/abc367_a

fn run(a: usize, b: usize, c: usize) -> &'static str {
    if b < c {
        if b < a && a < c {
            "No"
        } else {
            "Yes"
        }
    } else {
        if c < a && a < b {
            "Yes"
        } else {
            "No"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(21, 8, 14, "Yes"),
            TestCase(0, 21, 7, "No"),
            TestCase(10, 7, 17, "No")
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
