#[allow(dead_code)]
pub fn run(a: i32, b: i32, c: i32, d: i32) -> i32 {
    (a - c) * (b - d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, run(3, 2, 2, 1));
        assert_eq!(6, run(5, 5, 2, 3));
        assert_eq!(0, run(2, 4, 2, 4));
    }
}
