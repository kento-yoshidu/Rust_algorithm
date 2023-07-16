#[allow(dead_code)]
pub fn run(n: usize) -> usize {
    n * n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, run(2));
        assert_eq!(64, run(8));
        assert_eq!(10000, run(100));
    }
}
