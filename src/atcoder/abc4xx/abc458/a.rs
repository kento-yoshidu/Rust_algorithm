// https://atcoder.jp/contests/abc458/tasks/abc458_a

fn run(s: &str, n: usize) -> String {
    format!("{}", &s[n..(s.len() - n)])
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize, &'static str);

    #[test]
    fn abc458_a() {
        let tests = [
            TestCase("chemotherapy", 3, "mother"),
            TestCase("thermometer", 4, "mom"),
            TestCase("burger", 1, "urge"),
        ];

        for TestCase(s, n, expected) in tests {
            assert_eq!(run(s, n), expected);
        }
    }
}
