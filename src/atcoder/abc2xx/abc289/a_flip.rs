// https://atcoder.jp/contests/abc289/tasks/abc289_a

pub fn run(s: &str) -> String {
    s.chars().map(|c| {
        if c == '0' {
            '1'
        } else {
            '0'
        }
    }).collect()
}

fn run2(s: &str) -> String {
    s.chars()
        .map(|c| (c.to_digit(10).unwrap() ^ 1).to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("01", "10"),
            TestCase("1011", "0100"),
            TestCase("100100001", "011011110"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
            assert_eq!(run2(s), expected);
        }
    }
}
