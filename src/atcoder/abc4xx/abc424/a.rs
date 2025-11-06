// https://atcoder.jp/contests/abc424/tasks/abc424_a

fn run(a: usize, b: usize, c: usize) -> &'static str {
    if a == b || a == c || b == c {
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
    fn abc424_a() {
        let tests = [
            TestCase(4, 2, 4, "Yes"),
            TestCase(3, 4, 5, "No"),
            TestCase(10, 10, 10, "Yes"),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
