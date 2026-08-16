// https://atcoder.jp/contests/abc335/tasks/abc335_a

fn run(s: &str) -> String {
    format!("{}4", &s[0..s.len()-1])
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn abc335_a() {
        let tests = [
            TestCase("hello2023", "hello2024"),
            TestCase("worldtourfinals2023", "worldtourfinals2024"),
            TestCase("2023", "2024"),
            TestCase("20232023", "20232024"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
