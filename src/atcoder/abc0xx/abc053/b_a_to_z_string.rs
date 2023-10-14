// https://atcoder.jp/contests/abc016/tasks/abc016_2

pub fn run(s: String) -> usize {
    let a = &s.chars().position(|c| {
        c == 'A'
    }).unwrap();

    let z = &s.chars().rev().position(|c| {
        c == 'Z'
    }).unwrap();

    s.len() - z - a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(5, run(String::from("QWERTYASDFZXCV")));
        assert_eq!(4, run(String::from("ZABCZ")));
        assert_eq!(12, run(String::from("HASFJGHOGAKZZFEGA")));
    }
}
