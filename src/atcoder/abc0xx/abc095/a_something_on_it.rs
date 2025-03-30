// https://atcoder.jp/contests/abc095/tasks/abc095_a

fn run(s: &str) -> usize {
    700 + 100 * s.chars().filter(|c| { *c == 'o' }).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase("oxo", 900),
            TestCase("ooo", 1000),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
