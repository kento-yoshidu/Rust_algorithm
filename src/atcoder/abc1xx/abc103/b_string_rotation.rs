// https://atcoder.jp/contests/abc103/tasks/abc103_b

pub fn run(s: &str, t: &str) -> String {
    if (0..s.len()).any(|i| {
        s[i..].to_string() + &s[..i] == t
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
        assert_eq!(String::from("Yes"), run("kyoto", "tokyo"));
        assert_eq!(String::from("No"), run("abc", "arc"));
        assert_eq!(String::from("Yes"), run("aaaaaaaaaaaaaaab", "aaaaaaaaaaaaaaab"));
    }
}
