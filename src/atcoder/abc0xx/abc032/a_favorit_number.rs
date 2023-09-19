pub fn run(a: usize, b: usize, n: usize) -> usize {
    (n..).find(|i| {
        *i % a == 0 && *i % b == 0
    }).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(12, run(2, 3, 8));
        assert_eq!(2, run(2, 2, 2));
        assert_eq!(48, run(12, 8, 25));
    }
}
