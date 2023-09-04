// https://atcoder.jp/contests/abc289/tasks/abc289_a

pub fn run(s: String) -> String {
    s.chars().map(|c| {
        if c == '0' {
            '1'
        } else {
            '0'
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("10"), run(String::from("01")));
        assert_eq!(String::from("0100"), run(String::from("1011")));
        assert_eq!(String::from("011011110"), run(String::from("100100001")));
    }
}
