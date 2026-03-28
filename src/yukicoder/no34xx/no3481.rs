// https://yukicoder.me/problems/no/3481

fn run(n: usize) -> &'static str {
    if n / 100 + n % 10 == n / 10 % 10 {
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
    fn yuki_3481() {
        let tests = [
            TestCase(495, "Yes"),
            TestCase(999, "No"),
            TestCase(121, "Yes"),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
