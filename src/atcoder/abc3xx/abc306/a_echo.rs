// https://atcoder.jp/contests/abc306/tasks/abc306_a

pub fn run(_n: usize, s: String) -> String {
    let mut ans = String::new();

    for c in s.chars() {
        ans.push(c);
        ans.push(c);
    }

    ans
}

pub fn run2(_n: usize, s: String) -> String {
    s.chars().map(|c| {
        format!("{}{}", c, c)
    }).collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("bbeeggiinnnneerr"), run(8, String::from("beginner")));
        assert_eq!(String::from("aaaaaa"), run(3, String::from("aaa")));
    }

    #[test]
    fn test2() {
        assert_eq!(String::from("bbeeggiinnnneerr"), run2(8, String::from("beginner")));
        assert_eq!(String::from("aaaaaa"), run2(3, String::from("aaa")));
    }
}
