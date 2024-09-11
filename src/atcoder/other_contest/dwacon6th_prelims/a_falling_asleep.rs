// https://atcoder.jp/contests/dwacon6th-prelims/tasks/dwacon6th_prelims_a

pub fn run(n: usize, st: Vec<(&str, usize)>, s: &str) -> usize {
    let pos = st.iter()
        .position(|(str, _)| *str == s)
        .unwrap();

    (pos+1..n)
        .map(|i| st[i].1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(&'static str, usize)>, &'static str, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![("dwango", 2), ("sixth", 5), ("prelims", 25)], "dwango", 30),
            TestCase(1, vec![("abcde", 1000)], "abcde", 0),
            TestCase(15, vec![("ypnxn", 279), ("kgjgwx", 464), ("qquhuwq", 327), ("rxing", 549), ("pmuduhznoaqu", 832), ("dagktgdarveusju", 595), ("wunfagppcoi", 200), ("dhavrncwfw", 720), ("jpcmigg", 658), ("wrczqxycivdqn", 639), ("mcmkkbnjfeod", 992), ("htqvkgkbhtytsz", 130), ("twflegsjz", 467), ("dswxxrxuzzfhkp", 989), ("szfwtzfpnscgue", 958)], "pmuduhznoaqu", 6348),
        ];

        for TestCase(n, st, s, expected) in tests {
            assert_eq!(run(n, st, s), expected);
        }
    }
}
