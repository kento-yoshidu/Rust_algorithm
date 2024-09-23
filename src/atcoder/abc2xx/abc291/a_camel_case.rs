// https://atcoder.jp/contests/abc291/tasks/abc291_a

fn run(s: &str) -> usize {
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() == true {
            return i+1;
        }
    }

    unreachable!()
}

fn run2(s: &str) -> usize {
    s.chars()
        .position(|c| c.is_uppercase())
        .unwrap() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase("aBc", 2),
            TestCase("xxxxxxXxxx", 7),
            TestCase("Zz", 1),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
            assert_eq!(run2(s), expected);
        }
    }
}
