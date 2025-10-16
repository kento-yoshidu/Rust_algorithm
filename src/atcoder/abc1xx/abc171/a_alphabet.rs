// https://atcoder.jp/contests/abc171/tasks/abc171_a

fn run(a: char) -> char {
    if a.is_uppercase() {
        'A'
    } else {
        'a'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(char, char);

    #[test]
    fn abc171_a() {
        let tests = [
            TestCase('B', 'A'),
            TestCase('a', 'a'),
        ];

        for TestCase(a, expected) in tests {
            assert_eq!(run(a), expected);
        }
    }
}
