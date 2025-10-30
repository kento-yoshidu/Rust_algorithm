// https://atcoder.jp/contests/abc422/tasks/abc422_a

fn run(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();

    if chars[2] == '8' {
        format!("{}-1", chars[0].to_digit(10).unwrap() + 1)
    } else {
        format!("{}-{}", chars[0], chars[2].to_digit(10).unwrap() + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn abc422_a() {
        let tests = [
            TestCase("4-2", "4-3"),
            TestCase("1-8", "2-1"),
            TestCase("3-3", "3-4"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
