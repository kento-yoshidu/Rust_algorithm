// https://yukicoder.me/problems/no/597

fn run(_n: usize, s: Vec<&str>) -> String {
    s.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<&'static str>, &'static str);

    #[test]
    fn yuki_596() {
        let tests = [
            TestCase(2, vec!["con", "cat"], "concat"),
            TestCase(3, vec!["servalcat", "sandcat", "margaycat"], "servalcatsandcatmargaycat"),
            TestCase(4, vec!["ko", "un", "ko", "un"], "kounkoun"),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
