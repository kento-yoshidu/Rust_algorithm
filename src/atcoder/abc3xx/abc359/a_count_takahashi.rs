// https://atcoder.jp/contests/abc359/tasks/abc359_a

fn run(_n: usize, s: Vec<&str>) -> usize {
    s.into_iter()
        .filter(|str| {
            *str == "Takahashi"
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<&'static str>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec!["Aoki", "Takahashi", "Takahashi"], 2),
            TestCase(2, vec!["Aoki", "Aoki"], 0),
            TestCase(20, vec!["Aoki", "Takahashi", "Takahashi", "Aoki", "Aoki", "Aoki", "Aoki", "Takahashi", "Aoki", "Aoki", "Aoki", "Takahashi", "Takahashi", "Aoki", "Takahashi", "Aoki", "Aoki", "Aoki", "Aoki", "Takahashi"], 7),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
