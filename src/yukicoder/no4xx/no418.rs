// https://yukicoder.me/problems/no/418

fn run(s: &str) -> usize {
    s.chars()
        .filter(|n| *n == 'n')
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize);

    #[test]
    fn yuki_418() {
        let tests = [
            TestCase("mi--nminminminmi-------n", 5),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
