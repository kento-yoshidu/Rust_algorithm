// https://atcoder.jp/contests/abc292/tasks/abc292_a

fn run<'a>(s: &'a str) -> String {
    s.to_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn abc292_a() {
        let tests = [
            TestCase("abc", "ABC"),
            TestCase("a", "A"),
            TestCase("abcdefghjiklnmoqprstvuwxyz", "ABCDEFGHJIKLNMOQPRSTVUWXYZ"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
