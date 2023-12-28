// https://atcoder.jp/contests/arc055/tasks/arc055_a

pub fn run(n: usize) -> String {
    format!("1{}7", "0".repeat(n-1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("1000000007"), run(9));
        assert_eq!(String::from("1007"), run(3));
        assert_eq!(String::from("100000000000000000000000000000000000000000000000007"), run(50));
    }
}
