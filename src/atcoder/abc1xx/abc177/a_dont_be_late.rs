// https://atcoder.jp/contests/abc177/tasks/abc177_a

fn run(d: usize, t: usize, s: usize) -> &'static str {
    if d <= s * t {
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
    fn abc177_a() {
        let tests = [
            TestCase(1000, 15, 80, "Yes"),
            TestCase(2000, 20, 100, "Yes"),
            TestCase(10000, 1, 1, "No"),
        ];

        for TestCase(d, t, s, expected) in tests {
            assert_eq!(run(d, t, s), expected);
        }
    }
}
