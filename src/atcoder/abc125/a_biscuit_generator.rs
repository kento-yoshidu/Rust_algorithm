pub fn run(a: usize, b: usize, t: usize) -> usize {
    (t / a) * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(10, run(3, 5, 7));
        assert_eq!(6, run(3, 2, 9));
        assert_eq!(0, run(20, 20, 19));
    }
}
