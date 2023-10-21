// https://atcoder.jp/contests/abc017/tasks/abc017_2

pub fn run(s: String) -> String {
    if s.chars()
        .any(|c| {
            !['c', 'h', 'o', 'k', 'u'].contains(&c)
        }) {
            return String::from("NO")
        }

    if s.chars().last().unwrap() == 'c' {
        return String::from("NO")
    }

    if s.chars().nth(0).unwrap() == 'h' {
        return String::from("NO")
    }

    if s.chars().collect::<Vec<char>>()
        .windows(2)
        .any(|t| {
            (t[0] == 'c' && t[1] != 'h') || (t[0] != 'c' && t[1] == 'h')
        }) {
            return String::from("NO")
        } else {
            return String::from("YES")
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("YES"), run(String::from("chokuou")));
        assert_eq!(String::from("NO"), run(String::from("kuccho")));
        assert_eq!(String::from("NO"), run(String::from("atcoder")));
    }
}
