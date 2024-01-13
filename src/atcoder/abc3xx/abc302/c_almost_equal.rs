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

    #[test]
    fn test() {
        assert_eq!("Yes", run(4, 4, vec!["bbed", "abcd", "abed", "fbed"]));
        assert_eq!("No", run(2, 5, vec!["abcde", "abced"]));
        assert_eq!("Yes", run(8, 4, vec!["fast", "face", "cast", "race", "fact", "rice", "nice", "case"]));
        assert_eq!("No", run(8, 5, vec!["wvyhn", "mvyhc", "mvyhn", "wvehn", "kvumn", "wvpmn", "wveon", "wwumn"]));
    }
}
