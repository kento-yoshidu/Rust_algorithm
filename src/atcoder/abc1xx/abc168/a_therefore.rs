// https://atcoder.jp/contests/abc168/tasks/abc168_a

fn run(n: usize) -> &'static str {
    match n % 10 {
        2 | 4 | 5 | 7 | 9 => "hon",
        0 | 1 | 6 | 8 => "pon",
        3 => "bon",
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn abc168_a() {
        let tests = [
            TestCase(16, "pon"),
            TestCase(2, "hon"),
            TestCase(183, "bon"),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
