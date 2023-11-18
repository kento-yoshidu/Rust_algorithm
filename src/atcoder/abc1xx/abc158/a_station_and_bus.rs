// https://atcoder.jp/contests/abc158/tasks/abc158_a

pub fn run(s: String) -> String {
    let chars: Vec<char> = s.chars().collect();

    if chars[0] == chars[1] && chars[1] == chars[2] {
        String::from("No")
    } else {
        String::from("Yes")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Yes"), run(String::from("ABA")));
        assert_eq!(String::from("Yes"), run(String::from("BBA")));
        assert_eq!(String::from("No"), run(String::from("BBB")));
    }
}
