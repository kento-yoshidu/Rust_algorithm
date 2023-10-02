// https://atcoder.jp/contests/abc179/tasks/abc179_a

pub fn run(s: String) -> String {
    match s.chars().last() {
        Some('s') => s + "es",
        _ => s + "s"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("apples"), run(String::from("apple")));
        assert_eq!(String::from("buses"), run(String::from("bus")));
        assert_eq!(String::from("boxs"), run(String::from("box")));
    }
}
