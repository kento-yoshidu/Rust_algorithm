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

fn check2(s: String) -> bool {
    s.chars().eq(s.chars().rev())
}

fn run(n: usize, s: Vec<&str>) -> String {
    for i in 0..n {
        for j in i+1..n {
            if check(format!("{}{}", s[i], s[j]).chars()) == true {
                return String::from("Yes")
            }
        }
    }

    String::from("No")
}

pub fn run2(_n: usize, s: Vec<&str>) -> String {
    if s.iter()
        .permutations(2)
        .any(|v| check2(format!("{}{}", v[0], v[1])))
    {
        String::from("Yes")
    } else {
        String::from("No")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Yes"), run(5, vec!["ab", "ccef", "da", "a", "fe"]));
        assert_eq!(String::from("No"), run(3, vec!["a", "b", "aba"]));
        assert_eq!(String::from("Yes"), run(2, vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"]));
    }

    #[test]
    fn test2() {
        assert_eq!(String::from("Yes"), run2(5, vec!["ab", "ccef", "da", "a", "fe"]));
        assert_eq!(String::from("No"), run2(3, vec!["a", "b", "aba"]));
        assert_eq!(String::from("Yes"), run2(2, vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"]));
    }
}
