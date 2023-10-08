// https://atcoder.jp/contests/abc291/tasks/abc291_a

pub fn run(s: String) -> usize {
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() == true {
            return i+1;
        }
    }

    unreachable!()
}

pub fn run2(s: String) -> usize {
    s.chars().position(|c| {
        c.is_uppercase()
    }).unwrap() + 1
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

    #[test]
    fn test2() {
        assert_eq!(2, run2(String::from("aBc")));
        assert_eq!(7, run2(String::from("xxxxxxXxxx")));
        assert_eq!(1, run2(String::from("Zz")));
    }
}
