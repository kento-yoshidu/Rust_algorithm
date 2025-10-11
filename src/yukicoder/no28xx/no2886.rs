// https://yukicoder.me/problems/no/2886

fn run(d: usize) -> &'static str {
    if d <= 100 {
        "Milk"
    } else {
        "Difficult"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn yuki_2886() {
        let tests = [
            TestCase(29, "Milk"),
            TestCase(189, "Difficult"),
            TestCase(100, "Milk"),
        ];

        for TestCase(d, expected) in tests {
            assert_eq!(run(d), expected);
        }
    }
}
