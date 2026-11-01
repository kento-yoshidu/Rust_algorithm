// https://atcoder.jp/contests/abc042/tasks/abc042_a

fn run(a: usize, b: usize, c: usize) -> &'static str {
    if a * b * c == 5 * 7 * 5 {
        "YES"
    } else {
        "NO"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, &'static str);

    #[test]
    fn abc042_a() {
        let tests = [
            TestCase(5, 5, 7, "YES"),
            TestCase(7, 7, 5, "NO"),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
