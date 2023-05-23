pub fn run(r: usize) -> usize {
    r * r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, run(2));
        assert_eq!(10000, run(100));
    }
}
