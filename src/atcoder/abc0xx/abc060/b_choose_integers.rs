// https://atcoder.jp/contests/abc060/tasks/abc060_b

fn run(a: usize, b: usize, c: usize) -> &'static str {
    if (a..=a*b)
        .step_by(a)
        .any(|num| {
            num % b == c
        }) {
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
    fn test() {
        let tests = [
            TestCase(7, 5, 1, "YES"),
            TestCase(2, 2, 1, "NO"),
            TestCase(1, 100, 97, "YES"),
            TestCase(40, 98, 58, "YES"),
            TestCase(77, 42, 36, "NO"),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
