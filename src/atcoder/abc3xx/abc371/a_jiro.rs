// https://atcoder.jp/contests/abc371/tasks/abc371_a

fn run(ab: char, ac: char, bc: char) -> char {
    if ab == ac && ac == bc {
        'B'
    } else if ab == ac && ac != bc {
        'C'
    } else {
        'A'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(char, char, char, char);

    #[test]
    fn test() {
        let tests = [
            TestCase('<', '<', '<', 'B'),
            TestCase('<', '<', '>', 'C'),
        ];

        for TestCase(ab, ac, bc, expected) in tests {
            assert_eq!(run(ab, ac, bc), expected);
        }
    }
}
