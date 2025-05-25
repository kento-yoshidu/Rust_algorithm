//  https://atcoder.jp/contests/abc109/tasks/abc109_b

use itertools::Itertools;

fn run(_n: usize, w: Vec<&str>) -> &'static str {
    if !w.iter().all_unique() {
        return "No";
    }

    if w.windows(2)
        .all(|t| {
            t[0].chars().last().unwrap() == t[1].chars().nth(0).unwrap()
        }) {
            "Yes"
        } else {
            "No"
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<&'static str>, &'static str);

    #[test]
    fn abc109_b() {
        let tests = [
            TestCase(4, vec!["hoge", "english", "hoge", "enigma"], "No"),
            TestCase(9, vec!["basic", "c", "cpp", "php", "python", "nadesico", "ocaml", "lua", "assembly"], "Yes"),
            TestCase(8, vec!["a", "aa", "aaa", "aaaa", "aaaaa", "aaaaaa", "aaa", "aaaaaaa"], "No"),
            TestCase(3, vec!["abc", "arc", "agc"], "No"),
        ];

        for TestCase(n, w, expected) in tests {
            assert_eq!(run(n, w), expected);
        }
    }
}
