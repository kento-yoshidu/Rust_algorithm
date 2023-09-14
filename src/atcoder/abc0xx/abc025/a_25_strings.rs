// https://atcoder.jp/contests/abc025/tasks/abc025_a

pub fn run(s: String, n: usize) -> String {
    let vec: Vec<_> = s.chars().collect();

    let mut count = 0;
    let mut ans = String::new();

    for a in &vec {
        for b in &vec {
            count += 1;

            if count == n {
                ans.push(*a);
                ans.push(*b);
            }
        }
    }

    ans
}

pub fn run2(s: String, n: usize) -> String {
    let vec: Vec<_> = s.chars().collect();

    let mut ans = String::new();

    ans.push(vec[(n-1) / 5]);
    ans.push(vec[(n-1) % 5]);

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("bc"), run(String::from("abcde"), 8));
        assert_eq!(String::from("ue"), run(String::from("aeiou"), 22));
        assert_eq!(String::from("zz"), run(String::from("vwxyz"), 25));
    }

    #[test]
    fn test2() {
        assert_eq!(String::from("bc"), run2(String::from("abcde"), 8));
        assert_eq!(String::from("ue"), run2(String::from("aeiou"), 22));
        assert_eq!(String::from("zz"), run2(String::from("vwxyz"), 25));
    }
}
