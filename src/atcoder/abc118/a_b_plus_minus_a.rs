pub fn run(a: u8, b: u8) -> u8 {
    if b % a == 0 {
        return a + b
    }

    b - a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(16, run(4, 12));
        assert_eq!(12, run(8, 20));
        assert_eq!(2, run(1, 1));
    }
}
