// https://atcoder.jp/contests/abc179/tasks/abc179_a

fn run(s: &str) -> String {
    match s.chars().last().unwrap() {
        's' => format!("{}{}", s, "es"),
        _ => format!("{}{}", s, "s"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn abc179_a() {
        let tests = [
            TestCase("apple", "apples"),
            TestCase("bus", "buses"),
            TestCase("box", "boxs"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
