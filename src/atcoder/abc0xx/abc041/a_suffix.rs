// https://atcoder.jp/contests/abc041/tasks/abc041_a

#[allow(dead_code)]
pub fn run(s: String, i: usize) -> char {
    s.chars().nth(i - 1).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!('c', run(String::from("atcoder"), 3));
        assert_eq!('b', run(String::from("beginner"), 1));
        assert_eq!('t', run(String::from("contest"), 7));
    }
}
