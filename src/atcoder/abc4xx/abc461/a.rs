// https://atcoder.jp/contests/abc461/tasks/abc461_a

fn run(a: usize, d: usize) -> &'static str {
    if d >= a {
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
    fn abc461_a() {
        let tests = [
            TestCase(4, 5, "Yes"),
            TestCase(5, 5, "Yes"),
            TestCase(6, 5, "No"),
        ];

        for TestCase(a, d, expected) in tests {
            assert_eq!(run(a, d), expected);
        }
    }
}
