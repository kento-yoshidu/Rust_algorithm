// https://atcoder.jp/contests/abl/tasks/abl_a

fn run(k: usize) -> String {
    format!("{}", "ACL".repeat(k))
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, "ACLACLACL"),
        ];

        for TestCase(k, expected) in tests {
            assert_eq!(run(k), expected);
        }
    }
}
