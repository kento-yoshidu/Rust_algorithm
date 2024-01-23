// https://atcoder.jp/contests/abc268/tasks/abc268_b

pub fn run(s: &str, t: &str) -> String {
    if s.len() > t.len() {
        return String::from("No");
    }

    if s.chars()
        .zip(t.chars())
        .all(|(l, r)| {
            l == r
        }) {
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
        assert_eq!(String::from("Yes"), run("atco", "atcoder"));
        assert_eq!(String::from("No"), run("code", "atcoder"));
        assert_eq!(String::from("Yes"), run("abc", "abc"));
        assert_eq!(String::from("No"), run("aaaa", "aa"));
    }
}
