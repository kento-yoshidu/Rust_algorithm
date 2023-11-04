// https://atcoder.jp/contests/abc162/tasks/abc162_a

pub fn run(s: String) -> String {
    if s.chars()
        .any(|c| {
            c == '7'
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
        assert_eq!(String::from("Yes"), run(String::from("117")));
        assert_eq!(String::from("No"), run(String::from("123")));
        assert_eq!(String::from("Yes"), run(String::from("777")));
    }
}
