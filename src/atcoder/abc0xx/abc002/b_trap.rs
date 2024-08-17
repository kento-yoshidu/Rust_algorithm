// https://atcoder.jp/contests/abc002/tasks/abc002_2

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
    fn test() {
        let tests = [
            TestCase("chokudai", "chkd"),
            TestCase("okanemochi", "knmch"),
            TestCase("aoki", "k"),
            TestCase("mazushii", "mzsh"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
