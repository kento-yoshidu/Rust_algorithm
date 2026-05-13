// https://atcoder.jp/contests/abc315/tasks/abc315_a

fn run(s: &str) -> String {
    s.chars()
        .filter(|c| {
            !"aiueo".contains(*c)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn abc315_a() {
        let tests = [
            TestCase("atcoder", "tcdr"),
            TestCase("xyz", "xyz"),
            TestCase("aaaabbbbcccc", "bbbbcccc"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
