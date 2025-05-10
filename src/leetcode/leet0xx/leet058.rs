// https://leetcode.com/problems/length-of-last-word/description/

fn run(s: &str) -> usize {
    s.trim_end()
        .rsplit(' ')
        .find(|str| !str.is_empty())
        .map(|str| str.len())
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase("Hello World", 5),
            TestCase("   fly me   to   the moon  ", 4),
            TestCase("luffy is still joyboy", 6),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
