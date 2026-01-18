// https://atcoder.jp/contests/abc441/tasks/abc441_b

use std::collections::HashSet;

fn check(set: &HashSet<char>, a: &Vec<char>) -> bool {
    set.iter().all(|c| a.contains(c))
}

fn run(_n: usize, _m: usize, s: &str, t: &str, _q: usize, w: Vec<&str>) -> Vec<&'static str> {
    let s: Vec<char> = s.chars().collect();
    let t: Vec<char> = t.chars().collect();

    w.into_iter()
        .map(|str| {
            let set: HashSet<char> = str.chars().collect();

            let takahashi = check(&set, &s);
            let aoki = check(&set, &t);

            if takahashi & aoki {
                "Unknown"
            } else if !aoki {
                "Takahashi"
            } else {
                "Aoki"
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str, &'static str, usize, Vec<&'static str>, Vec<&'static str>);

    #[test]
    fn abc441_b() {
        let tests = [
            TestCase(6, 5, "ahikst", "aikot", 5, vec!["asahi", "okita", "kiai", "hash", "it"], vec!["Takahashi", "Aoki", "Unknown", "Takahashi", "Unknown"]),
            TestCase(7, 6, "ahiknst", "ahikos", 5, vec![ "kioki", "ohiki", "tashi", "nishi", "kashi"], vec!["Aoki", "Aoki", "Takahashi", "Takahashi", "Unknown"]),
            TestCase(13, 11, "defghiqsvwxyz", "acejmoqrtwx", 15, vec![ "qhsqzhd", "jcareec", "wwqxqew", "wxqxwex", "jxxrtwa", "trtqjxe", "sqyggse", "xxqwxew", "xewwxxw", "wwqxwex", "xqqxqwq", "qxxexxe", "teqeroc", "eeeqqee", "vxdevyy"], vec!["Takahashi", "Aoki", "Unknown", "Unknown", "Aoki", "Aoki", "Takahashi", "Unknown", "Unknown", "Unknown", "Unknown", "Unknown", "Aoki", "Unknown", "Takahashi"]),
        ];

        for TestCase(n, m, s, t, q, w, expected) in tests {
            assert_eq!(run(n, m, s, t, q, w), expected);
        }
    }
}
