#[allow(dead_code)]
pub fn run(a: i32, b: i32, c: i32) -> i32 {
    a * b / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(6, run(3, 4, 5));
        assert_eq!(30, run(5, 12, 13));
        assert_eq!(630, run(45, 28, 53));
    }
}
