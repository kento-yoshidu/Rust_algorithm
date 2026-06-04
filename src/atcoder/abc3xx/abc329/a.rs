// https://atcoder.jp/contests/abc329/tasks/abc329_a

fn run(s: &str) -> String {
    s.chars()
        .map(|c| {
            format!("{} ", c)
        })
        .collect::<String>()
        .trim_end()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn abc329_a() {
        let tests = [
            TestCase("ABC", "A B C"),
            TestCase("ZZZZZZZ", "Z Z Z Z Z Z Z"),
            TestCase("OOXXOO", "O O X X O O"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
