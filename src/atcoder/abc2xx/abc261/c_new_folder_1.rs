// https://atcoder.jp/contests/abc261/tasks/abc261_c

use std::collections::HashMap;

pub fn run(_n: usize, s: Vec<&str>) -> Vec<String> {
    let mut hash_map = HashMap::new();

    let mut ans = Vec::new();

    for str in s {
        match hash_map.get(str) {
            Some(num) => {
                let s = format!("{}({})", str, num);
                ans.push(s);
            },
            None => {
                ans.push(str.to_string());
            }
        }

        *hash_map.entry(str.to_string()).or_insert(0) += 1;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<&'static str>, Vec<&'static str>);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, vec!["newfile", "newfile", "newfolder", "newfile", "newfolder"], vec!["newfile", "newfile(1)", "newfolder", "newfile(2)", "newfolder(1)"]),
            TestCase(11, vec!["a", "a", "a", "a", "a", "a", "a", "a", "a", "a", "a"], vec!["a", "a(1)", "a(2)", "a(3)", "a(4)", "a(5)", "a(6)", "a(7)", "a(8)", "a(9)", "a(10)"]),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(expected, run(n, s));
        }
    }
}
