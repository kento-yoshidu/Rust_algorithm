// https://atcoder.jp/contests/abc307/tasks/abc307_b

use core::str::Chars;
use itertools::Itertools;

// 回文になっているか
fn check(mut iter: Chars) -> bool {
    match iter.next() {
        None => true,
        Some(lhs) => match iter.next_back() {
            None => true,
            Some(rhs) => match lhs == rhs {
                true => check(iter),
                false => false
            }
        }
    }
}

fn run(n: usize, s: &Vec<&str>) -> &'static str {
    for i in 0..n {
        for j in i+1..n {
            if check(format!("{}{}", s[i], s[j]).chars()) {
                return "Yes";
            }
        }
    }

    "No"
}

fn check2(s: String) -> bool {
    s.chars().eq(s.chars().rev())
}

fn run2(_n: usize, s: &Vec<&str>) -> &'static str {
    if s.into_iter()
        .permutations(2)
        .any(|v| check2(format!("{}{}", v[0], v[1])))
    {
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
    fn abc307_b() {
        let tests = [
            TestCase(5, vec!["ab", "ccef", "da", "a", "fe"], "Yes"),
            TestCase(3, vec!["a", "b", "aba"], "No"),
            TestCase(2, vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"], "Yes"),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, &s), expected);
            assert_eq!(run2(n, &s), expected);
        }
    }
}
