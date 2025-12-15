// https://atcoder.jp/contests/abc247/tasks/abc247_a

fn run(s: &str) -> String {
    format!("0{}", &s[0..3])
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn abc247_a() {
        let tests = [
            TestCase("1011", "0101"),
            TestCase("0000", "0000"),
            TestCase("1111", "0111"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
