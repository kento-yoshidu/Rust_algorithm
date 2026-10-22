// https://atcoder.jp/contests/abc016/tasks/abc016_2

fn run(s: Vec<&str>) -> usize {
    s.into_iter()
        .filter(|str| {
            str.chars().any(|c| c == 'r')
        }).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(Vec<&'static str>, usize);

    #[test]
    fn abc029_b() {
        let tests = [
            TestCase(vec!["january", "february", "march", "april", "may", "june", "july", "august", "september", "october", "november", "december"], 8),
            TestCase(vec!["rrrrrrrrrr", "srrrrrrrrr", "rsr", "ssr", "rrs", "srsrrrrrr", "rssrrrrrr", "sss", "rrr", "srr", "rsrrrrrrrr", "ssrrrrrrrr"], 11),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
