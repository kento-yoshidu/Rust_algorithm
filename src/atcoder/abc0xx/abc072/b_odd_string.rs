// https://atcoder.jp/contests/abc072/tasks/abc072_b

pub fn run(s: String) -> String {
    s.chars().step_by(2).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("acdr"), run(String::from("atcoder")));
        assert_eq!(String::from("aa"), run(String::from("aaaa")));
        assert_eq!(String::from("z"), run(String::from("z")));
        assert_eq!(String::from("fkoaaauh"), run(String::from("fukuokayamaguchi")));
    }
}
