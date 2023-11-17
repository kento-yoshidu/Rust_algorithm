// https://atcoder.jp/contests/abc154/tasks/abc154_b

pub fn run(s: &str) -> String {
    format!("{}", "x".repeat(s.len()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("xxxxxxx"), run("sardine"));
        assert_eq!(String::from("xxxx"), run("xxxx"));
        assert_eq!(String::from("xxxx"), run("gone"));
    }
}
