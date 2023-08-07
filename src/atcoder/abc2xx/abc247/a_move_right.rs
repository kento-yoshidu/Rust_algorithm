// https://atcoder.jp/contests/abc247/tasks/abc247_a

pub fn run(s: &str) -> String {
    format!("0{}", &s[0..3])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("0101"), run("1011"));
        assert_eq!(String::from("0000"), run("0000"));
        assert_eq!(String::from("0111"), run("1111"));
    }
}

