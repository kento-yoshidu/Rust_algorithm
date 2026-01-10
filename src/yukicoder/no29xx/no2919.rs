// https://yukicoder.me/problems/no/2919

fn run(n: usize, m: usize, k: usize) -> &'static str {
    if m + k <= n {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, &'static str);

    #[test]
    fn yuki_2919() {
        let tests = [
            TestCase(100, 70, 10, "Yes"),
            TestCase(16, 12, 4, "Yes"),
            TestCase(1000, 100000, 10000, "No"),
            TestCase(10, 29, 20, "No"),
        ];

        for TestCase(n, m, k, expected) in tests {
            assert_eq!(run(n, m, k), expected);
        }
    }
}
