// https://yukicoder.me/problems/no/543

fn run(a: char, b: char) -> (char, char) {
    (b, a)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(char, char, (char, char));

    #[test]
    fn yuki_536() {
        let tests = [
            TestCase('T', 'F', ('F', 'T')),
            TestCase('F', 'F', ('F', 'F')),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
