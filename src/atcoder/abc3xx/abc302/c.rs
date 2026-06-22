// https://atcoder.jp/contests/abc302/tasks/abc302_c

use itertools::Itertools;

fn run(n: usize, m: usize, s: Vec<&str>) -> &'static str {
    for vec in s.iter().permutations(n) {
        let mut v = Vec::new();

        for str in vec.windows(2) {
            if str[0].chars().zip(str[1].chars())
                .filter(|t| {
                    t.0 == t.1
                })
                .count() >= m-1 {
                    v.push(true);
                } else {
                    v.push(false);
                }
        }

        if v.iter().all(|b| *b) {
            return "Yes";
        }
    }

    "No"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<&'static str>, &'static str);

    #[test]
    fn abc302_c() {
        let tests = [
            TestCase(4, 4, vec!["bbed", "abcd", "abed", "fbed"], "Yes"),
            TestCase(2, 5, vec!["abcde", "abced"], "No"),
            TestCase(8, 4, vec!["fast", "face", "cast", "race", "fact", "rice", "nice", "case"], "Yes"),
            TestCase(8, 5, vec!["wvyhn", "mvyhc", "mvyhn", "wvehn", "kvumn", "wvpmn", "wveon", "wwumn"], "No"),
        ];

        for TestCase(n, m, s, expected) in tests {
            assert_eq!(run(n, m, s), expected);
        }
    }
}
