pub fn run(d: i32) -> f64 {
    d as f64 / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run(1000), 10.0);
        assert_eq!(run(50), 0.5);
        assert_eq!(run(3141), 31.41);
    }
}
