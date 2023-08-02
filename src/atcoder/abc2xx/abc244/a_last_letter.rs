// https://atcoder.jp/contests/abc244/tasks/abc244_a

pub fn run(n: usize, s: String) -> char {
    s.chars().nth(n-1).unwrap()
}

pub fn run2(_n: usize, s: String) -> char {
    s.chars().rev().nth(0).unwrap()
}

pub fn run3(_n: usize, s: String) -> char {
    s.chars().last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!('e', run(5, String::from("abcde")));
        assert_eq!('a', run(1, String::from("a")));
    }

    #[test]
    fn test2() {
        assert_eq!('e', run2(5, String::from("abcde")));
        assert_eq!('a', run2(1, String::from("a")));
    }

    #[test]
    fn test3() {
        assert_eq!('e', run3(5, String::from("abcde")));
        assert_eq!('a', run3(1, String::from("a")));
    }
}
