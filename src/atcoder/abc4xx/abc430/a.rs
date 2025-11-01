// https://atcoder.jp/contests/abc430/tasks/abc430_a

fn run(a: usize, b: usize, c: usize, d: usize) -> &'static str {
    if c >= a && b >= d {
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
    fn abc430_a() {
        let tests = [
            TestCase(10, 20, 30, 40, "No"),
            TestCase(10, 20, 30, 4, "Yes"),
            TestCase(100, 100, 1, 1, "No"),
        ];

        for TestCase(a, b, c, d, expected) in tests {
            assert_eq!(run(a, b, c, d), expected);
        }
    }
}
