// https://atcoder.jp/contests/abc307/tasks/abc307_b

use core::str::Chars;

// 回文になっているか
#[allow(unused_mut)]
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

#[allow(dead_code)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Yes"), run(5, vec!["ab", "ccef", "da", "a", "fe"]));
        assert_eq!(String::from("No"), run(3, vec!["a", "b", "aba"]));
        assert_eq!(String::from("Yes"), run(5, vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"]));
    }
}
