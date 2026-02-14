// https://yukicoder.me/problems/no/2862

fn run(n: usize, s: &str) -> &'static str {
    if s.contains("404") {
        "Found"
    } else {
        "NotFound"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, &'static str);

    #[test]
    fn yuki_2862() {
        let tests = [
            TestCase(10, "1843404261", "Found"),
            TestCase(10, "45616572034", "NotFound"),
            TestCase(10, "0404404404", "Found"),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
