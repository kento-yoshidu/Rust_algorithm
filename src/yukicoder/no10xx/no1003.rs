// https://yukicoder.me/problems/no/1003

fn run(n: usize) -> &'static str {
    if n % 6 == 0 {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn yuki_1003() {
        let tests = [
            TestCase(8, "No"),
            TestCase(6, "Yes"),
            TestCase(65536, "No"),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
