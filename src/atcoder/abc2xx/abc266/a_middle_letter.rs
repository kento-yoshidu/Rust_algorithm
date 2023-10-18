// https://atcoder.jp/contests/abc266/tasks/abc266_a

pub fn run(s: String) -> char {
    s.chars().nth(s.len()/2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!('o', run(String::from("atcoder")));
        assert_eq!('a', run(String::from("a")));
    }
}
