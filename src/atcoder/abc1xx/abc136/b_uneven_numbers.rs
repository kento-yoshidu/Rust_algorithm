#[allow(dead_code)]
pub fn run(n: usize) -> usize {
    (1..=n).filter(|i| i.to_string().len() % 2 != 0).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(9, run(11));
        assert_eq!(46, run(136));
        assert_eq!(90909, run(100000));
    }
}
