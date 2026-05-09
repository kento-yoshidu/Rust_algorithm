// https://atcoder.jp/contests/abc312/tasks/abc312_a

fn run(s: &str) -> &'static str {
    if ["ACE", "BDF", "CEG", "DFA", "EGB", "FAC", "GBD"].contains(&s) {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn abc312_a() {
        let tests = [
            TestCase("ABC", "No"),
            TestCase("FAC", "Yes"),
            TestCase("XYX", "No"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
