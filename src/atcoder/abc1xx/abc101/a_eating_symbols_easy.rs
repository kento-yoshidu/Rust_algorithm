// https://atcoder.jp/contests/abc101/tasks/abc101_a

pub fn run(s: &str) -> isize {
    s.chars().map(|c| {
        if c == '+' {
            1
        } else {
            -1
        }
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase("+-++", 2),
            TestCase("-+--", -2),
            TestCase("----", -4),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
