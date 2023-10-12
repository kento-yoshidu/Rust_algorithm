// https://atcoder.jp/contests/abc258/tasks/abc258_a

pub fn run(n: usize) -> String {
    if n < 60 {
        format!("21:{:02}", n)
    } else {
        format!("22:{:02}", n - 60)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("21:45"), run(45));
        assert_eq!(String::from("22:00"), run(60));
        assert_eq!(String::from("22:01"), run(61));
        assert_eq!(String::from("22:30"), run(90));
        assert_eq!(String::from("22:40"), run(100));
    }
}
