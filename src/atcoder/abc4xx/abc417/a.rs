// https://atcoder.jp/contests/abc417/tasks/abc417_a

fn run(n: usize, a: usize, b: usize, s: &str) -> String {
    format!("{}", &s[a..=(n-b-1)])
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, &'static str, &'static str);

    #[test]
    fn abc417_a() {
        let tests = [
            TestCase(7, 1, 3, "atcoder", "tco"),
            TestCase(1, 0, 0, "a", "a"),
            TestCase(20, 4, 8, "abcdefghijklmnopqrst", "efghijkl"),
        ];

        for TestCase(n, a, b, s, expected) in tests {
            assert_eq!(run(n, a, b, s), expected);
        }
    }
}
