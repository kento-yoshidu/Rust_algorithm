pub fn run(a: i32, b: i32) -> i32 {
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(10, run(2, 5));
        assert_eq!(10000, run(100, 100));
    }
}
