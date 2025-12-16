// https://yukicoder.me/problems/no/882

fn run(a: usize, b: usize) -> &'static str {
    if a % b == 0 {
        "YES"
    } else {
        "NO"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str);

    #[test]
    fn yuki_882() {
        let tests = [
            TestCase(12, 3, "YES"),
            TestCase(10, 3, "NO"),
            TestCase(13, 13, "YES"),
            TestCase(1, 1, "YES"),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
