pub fn run(a: i32, b: i32) -> f64 {
    ((a - b) as f64 / a as f64) * 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(20.0, run(100, 80));
        assert_eq!(14.285714285714285, run(7, 6));
        assert_eq!(0.00100001000010000100, run(99999, 99998));

    }
}