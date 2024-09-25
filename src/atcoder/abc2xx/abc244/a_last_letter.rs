// https://atcoder.jp/contests/abc244/tasks/abc244_a

fn run(n: usize, s: &'static str) -> char {
    s.chars().nth(n-1).unwrap()
}

fn run2(_n: usize, s: &'static str) -> char {
    s.chars().rev().nth(0).unwrap()
}

fn run3(_n: usize, s: &'static str) -> char {
    s.chars().last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, char);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, "abcde", 'e'),
            TestCase(1, "a", 'a'),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
            assert_eq!(run2(n, s), expected);
            assert_eq!(run3(n, s), expected);
        }
    }
}
