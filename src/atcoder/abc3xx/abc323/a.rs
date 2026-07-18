// https://atcoder.jp/contests/abc323/tasks/abc323_a

fn run(s: &str) -> &'static str {
    if s.chars()
        .skip(1)
        .step_by(2)
        .all(|c| { c == '0' })
    {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn abc323_a() {
        let tests = [
            TestCase("1001000000001010", "No"),
            TestCase("1010100000101000", "Yes"),
            TestCase("1111111111111111", "No"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
