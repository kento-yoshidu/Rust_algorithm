// https://atcoder.jp/contests/abc389/tasks/abc389_a

fn run(s: &str) -> u32 {
    let str: Vec<char> = s.chars().collect();

    str[0].to_digit(10).unwrap() * str[2].to_digit(10).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, u32);

    #[test]
    fn test() {
        let tests = [
            TestCase("3x8", 24),
            TestCase("9x9", 81),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
