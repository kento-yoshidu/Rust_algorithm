// https://atcoder.jp/contests/abc280/tasks/abc280_c

fn run(s: &str, t: &str) -> usize {
    s.chars()
        .zip(t.chars())
        .position(|(l, r)| {
            l != r
        })
        .unwrap_or(s.len()) + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str, usize);

    #[test]
    fn abc280_c() {
        let tests = [
            TestCase("atcoder", "atcorder", 5),
            TestCase("million", "milllion", 5),
            TestCase("vvwvw", "vvvwvw", 3),
        ];

        for TestCase(s, t, expected) in tests {
            assert_eq!(run(s, t), expected);
        }
    }
}
