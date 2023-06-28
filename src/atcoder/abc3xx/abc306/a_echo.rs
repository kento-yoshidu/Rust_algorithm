// https://atcoder.jp/contests/abc306/tasks/abc306_a

pub fn run(_n: usize, s: String) -> String {
    let mut ans = String::new();

    for c in s.chars() {
        ans.push(c);
        ans.push(c);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("bbeeggiinnnneerr"), run(8, String::from("beginner")));
        assert_eq!(String::from("aaaaaa"), run(8, String::from("aaa")));
    }
}
