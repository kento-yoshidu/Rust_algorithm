// https://atcoder.jp/contests/abc291/tasks/abc291_a

#[allow(dead_code)]
pub fn run(s: String) -> usize {
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() == true {
            return i+1;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(String::from("aBc")));
        assert_eq!(7, run(String::from("xxxxxxXxxx")));
        assert_eq!(1, run(String::from("Zz")));
    }
}
