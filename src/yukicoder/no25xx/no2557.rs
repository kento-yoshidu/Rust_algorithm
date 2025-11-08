// https://yukicoder.me/problems/no/2557

fn run(n: usize) -> &'static str {
    if n < 1200 {
        "green"
    } else {
        "difficult"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn yuki_2557() {
        let tests = [
            TestCase(800, "green"),
            TestCase(1600, "difficult"),
            TestCase(1200, "difficult"),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
