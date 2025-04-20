// https://atcoder.jp/contests/abc402/tasks/abc402_a

fn run(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_uppercase())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("AtCoderBeginnerContest", "ACBC"),
            TestCase("PaymentRequired", "PR"),
            TestCase("program", ""),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
