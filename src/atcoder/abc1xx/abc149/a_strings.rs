// https://atcoder.jp/contests/abc149/tasks/abc149_a

pub fn run(s: &str, t: &str) -> String {
    format!("{}{}", t, s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("atcoder"), run("oder", "atc"));
        assert_eq!(String::from("humuhumu"), run("humu", "humu"));
    }
}
