// https://yukicoder.me/problems/no/494

fn run(s: &str) -> char {
    let chars: Vec<char> = "yukicoder".chars().collect();

    chars[s.chars().position(|c| c == '?').unwrap()]
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, char);

    #[test]
    fn yuki_494() {
        let tests = [
            TestCase("yu?icoder", 'k'),
            TestCase("yukico?er", 'd'),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
