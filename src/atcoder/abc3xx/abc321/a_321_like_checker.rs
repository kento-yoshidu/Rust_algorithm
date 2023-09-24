// https://atcoder.jp/contests/abc321/tasks/abc321_a

pub fn run(s: String) -> String {
    let chars: Vec<char> = s.chars().collect();

    for i in 1..chars.len() {
        if chars[i] >= chars[i-1] {
            return String::from("No");
        }
    }

    String::from("Yes")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Yes"), run(String::from("321")));
        assert_eq!(String::from("No"), run(String::from("123")));
        assert_eq!(String::from("Yes"), run(String::from("1")));
        assert_eq!(String::from("No"), run(String::from("86411")));
    }
}
