#[allow(dead_code)]
pub fn run2(n: usize) -> String {
    format!("{:0>1$b}", n, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("0000001101"), run2(13));
        assert_eq!(String::from("0000100101"), run2(37));
        assert_eq!(String::from("1111101000"), run2(1000));
    }
}
