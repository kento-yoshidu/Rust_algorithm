// https://atcoder.jp/contests/abc056/tasks/abc056_a

fn run(a: char, b: char) -> char {
    if a == 'H' {
        b
    } else if b == 'H' {
        'D'
    } else {
        'H'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(char, char, char);

    #[test]
    fn test() {
        let tests = [
            TestCase('H', 'H', 'H'),
            TestCase('D', 'H', 'D'),
            TestCase('D', 'D', 'H'),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
