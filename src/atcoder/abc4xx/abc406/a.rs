// https://atcoder.jp/contests/abc406/tasks/abc406_a

fn run(a: usize, b: usize, c: usize, d: usize) -> &'static str {
    if a * 60 + b > c * 60 + d {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(22, 40, 22, 30, "Yes"),
            TestCase(22, 40, 22, 45, "No"),
            TestCase(12, 0, 11, 30, "Yes"),
        ];

        for TestCase(a, b, c, d, expected) in tests {
            assert_eq!(run(a, b, c, d), expected);
        }
    }
}
