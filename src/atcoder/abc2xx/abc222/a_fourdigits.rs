// https://atcoder.jp/contests/abc222/tasks/abc222_a

pub fn run(n: usize) -> String {
    format!("{:04}", n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("0321"), run(321));
        assert_eq!(String::from("7777"), run(7777));
        assert_eq!(String::from("0001"), run(1));
    }
}
