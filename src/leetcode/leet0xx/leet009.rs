// https://leetcode.com/problems/palindrome-number/description/

fn check(s: String) -> bool {
    s.chars().eq(s.chars().rev())
}

fn run(x: isize) -> bool {
    check(x.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, bool);

    #[test]
    fn test() {
        let tests = [
            TestCase(121, true),
            TestCase(-121, false),
            TestCase(10, false),
        ];

        for TestCase(x, expected) in tests {
            assert_eq!(run(x), expected);
        }
    }
}
