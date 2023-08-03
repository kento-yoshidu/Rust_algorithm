// https://atcoder.jp/contests/abc068/tasks/abc068_a

#[allow(dead_code)]
pub fn run(s: &str) -> String {
    String::from("ABC") + s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("ABC100"), run("100"));
        assert_eq!(String::from("ABC425"), run("425"));
        assert_eq!(String::from("ABC999"), run("999"));
    }
}
