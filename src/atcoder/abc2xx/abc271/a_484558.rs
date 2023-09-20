// https://atcoder.jp/contests/abc272/tasks/abc272_a

pub fn run(n: usize) -> String {
    format!("{:0>2X}", n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("63"), run(99));
        assert_eq!(String::from("0C"), run(12));
        assert_eq!(String::from("00"), run(0));
        assert_eq!(String::from("FF"), run(255));
    }
}
