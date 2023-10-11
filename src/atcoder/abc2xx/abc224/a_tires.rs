// https://atcoder.jp/contests/abc224/tasks/abc224_a

pub fn run(s: String) -> String {
    if s.chars().last().unwrap() == 'r' {
        String::from("er")
    } else {
        String::from("ist")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!("er", run(String::from("atcoder")));
        assert_eq!("ist", run(String::from("tourist")));
        assert_eq!("er", run(String::from("er")));
    }
}
