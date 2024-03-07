// https://atcoder.jp/contests/hhkb2020/tasks/hhkb2020_a

pub fn run(s: char, t: char) -> String {
    if s == 'Y' {
        t.to_uppercase().to_string()
    } else {
        t.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(char, char, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase('Y', 'a', "A"),
            TestCase('N', 'b', "b"),
        ];

        for TestCase(s, t, expected) in tests {
            assert_eq!(run(s, t), expected);
        }
    }
}
