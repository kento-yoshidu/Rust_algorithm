// https://atcoder.jp/contests/abc164/tasks/abc164_a

pub fn run(s: usize, w: usize) -> String {
    if w >= s {
        String::from("unsafe")
    } else {
        String::from("safe")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("unsafe"), run(4, 5));
        assert_eq!(String::from("safe"), run(100, 2));
        assert_eq!(String::from("unsafe"), run(10, 10));
    }
}
