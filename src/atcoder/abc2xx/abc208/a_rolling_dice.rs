// https://atcoder.jp/contests/abc206/tasks/abc206_a

fn run(a: usize, b: usize) -> &'static str {
    if a * 6 >= b && a != 0 {
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
    fn abc208_a() {
        let tests = [
            TestCase(2, 11, "Yes"),
            TestCase(2, 13, "No"),
            TestCase(100, 600, "Yes"),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
