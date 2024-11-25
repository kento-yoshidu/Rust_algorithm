// https://atcoder.jp/contests/abc041/tasks/abc041_a

fn run(s: &str, i: usize) -> char {
    s.chars().nth(i - 1).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize, char);

    #[test]
    fn test() {
        let tests = [
            TestCase("atcoder", 3, 'c'),
            TestCase("beginner", 1, 'b'),
            TestCase("contest", 7, 't'),
        ];

        for TestCase(s, i, expected) in tests {
            assert_eq!(run(s, i), expected);
        }
    }
}
