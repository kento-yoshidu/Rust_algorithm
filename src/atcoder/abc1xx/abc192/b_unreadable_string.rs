// https://atcoder.jp/contests/abc192/tasks/abc192_b

fn run(s: &str) -> &'static str {
    if s.chars()
        .enumerate()
        .all(|(i, c)| {
            if i % 2 == 0 {
                c.is_lowercase()
            } else {
                c.is_uppercase()
            }
        }) {
            "Yes"
        } else {
            "No"
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn abc192_b() {
        let tests = [
            TestCase("dIfFiCuLt", "Yes"),
            TestCase("eASY", "No"),
            TestCase("a", "Yes"),
            TestCase("A", "No"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
