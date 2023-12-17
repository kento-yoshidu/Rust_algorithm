// https://atcoder.jp/contests/abc333/tasks/abc333_a

pub fn run(n :usize) -> String {
    n.to_string().repeat(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("1"), run(1));
        assert_eq!(String::from("22"), run(2));
        assert_eq!(String::from("333"), run(3));
        assert_eq!(String::from("999999999"), run(9));
    }
}
