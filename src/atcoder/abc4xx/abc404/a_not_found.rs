// https://atcoder.jp/contests/abc404/tasks/abc404_a

fn run(s: &str) -> char {
    ('a'..='z')
        .find(|c| !s.contains(*c))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, char);

    #[test]
    fn test() {
        let tests = [
            TestCase("a", 'b'),
            TestCase("abcdfhijklmnopqrstuvwxyz", 'e'),
            TestCase("qazplwsxokmedcijnrfvuhbgt", 'y'),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
