// https://atcoder.jp/contests/abc446/tasks/abc446_a

fn run(s: &str) -> String {
    format!("Of{}", s.to_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn abc446_a() {
        let tests = [
            TestCase("Glen", "Ofglen"),
            TestCase("I", "Ofi"),
            TestCase("Fred", "Offred"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
