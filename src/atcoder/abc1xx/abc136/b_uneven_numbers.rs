// https://atcoder.jp/contests/abc136/tasks/abc136_b

fn run(n: usize) -> usize {
    (1..=n)
        .filter(|i| i.to_string().len() % 2 != 0)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn abc136_b() {
        let tests = [
            TestCase(11, 9),
            TestCase(136, 46),
            TestCase(100000, 90909),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
