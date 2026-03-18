// https://atcoder.jp/contests/abc266/tasks/abc266_a

fn run(s: &str) -> char {
    s.chars().nth(s.len()/2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, char);

    #[test]
    fn abc266_a() {
        let tests = [
            TestCase("atcoder", 'o'),
            TestCase("a", 'a'),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
