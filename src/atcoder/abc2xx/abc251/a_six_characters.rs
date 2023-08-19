// https://atcoder.jp/contests/abc251/tasks/abc251_a

pub fn run(s: &str) -> String {
    s.repeat(6 / s.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("wwwwww"), run("ww"));
        assert_eq!(String::from("abcabc"), run("abc"));
    }
}
