// https://atcoder.jp/contests/abc442/tasks/abc442_a

fn run(s: &str) -> usize {
    s.chars()
        .filter(|c| *c == 'i' || *c == 'j')
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize);

    #[test]
    fn abc442_a() {
        let tests = [
            TestCase("aiiaj", 3),
            TestCase("abcedfgh", 0),
            TestCase("jjjjjj", 6),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
