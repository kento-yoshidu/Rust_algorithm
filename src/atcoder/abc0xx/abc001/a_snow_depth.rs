#[allow(dead_code)]
pub fn run(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(5, run(15, 10));
        assert_eq!(0, run(0, 0));
        assert_eq!(-15, run(5, 20));
    }
}
