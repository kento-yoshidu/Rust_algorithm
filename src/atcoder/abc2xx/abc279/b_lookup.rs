// https://atcoder.jp/contests/abc182/tasks/abc182_c

fn run(s: &str, t: &str) -> &'static str {
    if s.contains(t) {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str, &'static str);

    #[test]
    fn abc279_b() {
        let tests = [
            TestCase("voltage", "tag", "Yes"),
            TestCase("atcoder", "ace", "No"),
            TestCase("gorilla", "gorillagorillagorilla", "No"),
            TestCase("toyotasystems", "toyotasystems", "Yes"),
        ];

        for TestCase(s, t, expected) in tests {
            assert_eq!(run(s, t), expected);
        }
    }
}
