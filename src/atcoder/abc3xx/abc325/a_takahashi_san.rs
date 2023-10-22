// https://atcoder.jp/contests/abc325/tasks/abc325_a

pub fn run(s: String, _: String) -> String {
    format!("{} san", s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Takahashi san"), run(String::from("Takahashi"), String::from("Chokudai")));
        assert_eq!(String::from("K san"), run(String::from("K"), String::from("Eyence")));
    }
}
