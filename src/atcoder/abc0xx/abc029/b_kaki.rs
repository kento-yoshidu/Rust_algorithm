// https://atcoder.jp/contests/abc016/tasks/abc016_2

fn run(s: Vec<&str>) -> usize {
    s.iter()
        .filter(|str| {
            str.chars().any(|c| c == 'r')
        }).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(Vec<&'static str>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(vec!["january", "february", "march", "april", "may", "june", "july", "august", "september", "october", "november", "december"], 8),
            TestCase(vec!["rrrrrrrrrr", "srrrrrrrrr", "rsr", "ssr", "rrs", "srsrrrrrr", "rssrrrrrr", "sss", "rrr", "srr", "rsrrrrrrrr", "ssrrrrrrrr"], 11),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}

// https://atcoder.jp/contests/abc029/submissions/42272316
