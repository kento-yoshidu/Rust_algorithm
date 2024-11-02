// https://atcoder.jp/contests/abc013/tasks/abc013_1

fn run(x: char) -> u8 {
    (x as u8 - 'A' as u8) + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(char, u8);

    #[test]
    fn test() {
        let tests = [
            TestCase('A', 1),
            TestCase('B', 2),
            TestCase('C', 3),
            TestCase('D', 4),
        ];

        for TestCase(x, expected) in tests {
            assert_eq!(run(x), expected);
        }
    }
}
