// https://atcoder.jp/contests/abc385/tasks/abc385_a

fn run(a: usize, b: usize, c: usize) -> &'static str {
    if a == b && b == c {
        "Yes"
    } else if a + b == c || b + c == a || a + c == b {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 8, 5, "Yes"),
            TestCase(2, 2, 2, "Yes"),
            TestCase(1, 2, 4, "No"),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
