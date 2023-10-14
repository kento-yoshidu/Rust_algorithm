// https://atcoder.jp/contests/abc230/tasks/abc230_a

pub fn run(n: usize) -> String {
    if n < 42 {
        format!("AGC{:03}", n)
    } else {
        format!("AGC{:03}", n+1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("AGC043"), run(42));
        assert_eq!(String::from("AGC019"), run(19));
        assert_eq!(String::from("AGC001"), run(1));
        assert_eq!(String::from("AGC051"), run(50));
    }
}
