// https://atcoder.jp/contests/abc016/tasks/abc016_2

pub fn run(s: Vec<&str>) -> usize {
    s.iter().filter(|str| {
        str.chars().any(|c| c == 'r')
    }).count()
}

// https://atcoder.jp/contests/abc029/submissions/42272316

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(8, run(vec!["january", "february", "march", "april", "may", "june", "july", "august", "september", "october", "november", "december"]));
        assert_eq!(11, run(vec!["rrrrrrrrrr", "srrrrrrrrr", "rsr", "ssr", "rrs", "srsrrrrrr", "rssrrrrrr", "sss", "rrr", "srr", "rsrrrrrrrr", "ssrrrrrrrr"]));
    }
}
