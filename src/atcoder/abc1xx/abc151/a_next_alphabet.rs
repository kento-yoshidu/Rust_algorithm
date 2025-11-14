// https://atcoder.jp/contests/abc151/tasks/abc151_a

fn run(c: char) -> char {
    (c as u8 + 1) as char
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(char, char);

    #[test]
    fn abc151_a() {
        let tests = [
            TestCase('a', 'b'),
            TestCase('y', 'z'),
        ];

        for TestCase(c, expected) in tests {
            assert_eq!(run(c), expected);
        }
    }
}
