// https://atcoder.jp/contests/abc322/tasks/abc322_b

pub fn run(n: usize, m: usize, s: String, t: String) -> isize {
    if (s == t) || (&t[0..n] == s && &t[m-n..] == s) {
        0
    } else if &t[0..n] == s  {
        1
    } else if &t[m-n..] == s {
        2
    } else {
        3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, run(3, 7, String::from("abc"), String::from("abcdefg")));
        assert_eq!(2, run(3, 4, String::from("abc"), String::from("aabc")));
        assert_eq!(3, run(3, 3, String::from("abc"), String::from("xyz")));
        assert_eq!(0, run(3, 3, String::from("aaa"), String::from("aaa")));
    }
}
