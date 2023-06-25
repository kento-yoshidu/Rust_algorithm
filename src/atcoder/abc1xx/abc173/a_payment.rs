#[allow(dead_code)]
pub fn run(n: i32) -> i32 {
    if n % 1000 == 0 {
        return 0;
    }

    (n as f64 / 1000.0).ceil() as i32 * 1000 - n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(100, run(1900));
        assert_eq!(0, run(3000));
        assert_eq!(475, run(3525));
    }
}
