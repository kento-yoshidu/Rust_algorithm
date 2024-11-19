// https://atcoder.jp/contests/arc050/tasks/arc050_a

fn run(c1: char, c2: char) -> &'static str {
    if c1.to_ascii_lowercase() == c2 {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(char, char, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase('A', 'a', "Yes"),
            TestCase('B', 'c', "No"),
            TestCase('Z', 'z', "Yes"),
        ];

        for TestCase(c1, c2, expected) in tests {
            assert_eq!(run(c1, c2), expected);
        }
    }
}
