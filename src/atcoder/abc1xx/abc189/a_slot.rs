// https://atcoder.jp/contests/abc189/tasks/abc189_a

pub fn run(s: String) -> String {
    let mut c = s.chars();

    let l = c.next().unwrap();
    let m = c.next().unwrap();
    let r = c.next().unwrap();

    if l == m && m == r {
        String::from("Won")
    } else {
        String::from("Lost")
    }
}

pub fn run2(s: String) -> String {
    let mut chars: Vec<char> = s.chars().collect();

    chars.dedup();

    if chars.len() == 1 {
        String::from("Won")
    } else {
        String::from("Lost")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Won"), run(String::from("SSS")));
        assert_eq!(String::from("Lost"), run(String::from("WVW")));
    }

    #[test]
    fn test2() {
        assert_eq!(String::from("Won"), run2(String::from("SSS")));
        assert_eq!(String::from("Lost"), run2(String::from("WVW")));
    }
}
