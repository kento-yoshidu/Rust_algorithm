// https://atcoder.jp/contests/abc284/tasks/abc284_a

fn run(_n: usize, s: Vec<&str>) -> Vec<&str> {
    s.into_iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<&'static str>, Vec<&'static str>);

    #[test]
    fn abc284_a() {
        let tests = [
            TestCase(3, vec!["Takahashi", "Aoki", "Snuke"], vec!["Snuke", "Aoki", "Takahashi"]),
            TestCase(4, vec!["2023", "Year", "New", "Happy"], vec!["Happy", "New", "Year", "2023"]),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
