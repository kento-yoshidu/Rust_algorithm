// https://atcoder.jp/contests/abc320/tasks/abc320_b

fn check(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

pub fn run(s: String) -> usize {
    let mut ans = 1;

    for i in 0..s.len() {
        for j in i+1..s.len() {
            if check(&s[i..=j]) {
                ans = ans.max(j+1 - i);
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(5, run(String::from("TOYOTA")));
        assert_eq!(1, run(String::from("ABCDEFG")));
        assert_eq!(10, run(String::from("AAAAAAAAAA")));
        assert_eq!(100, run(String::from("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA")));
    }
}
