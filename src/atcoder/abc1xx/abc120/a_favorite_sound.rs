#[allow(dead_code)]
pub fn run(a: i32, b: i32, c: i32) -> i32 {
    if b / a >= c {
        c
    } else {
        b / a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, run(2, 11, 4));
        assert_eq!(3, run(3, 9, 5));
        assert_eq!(0, run(100, 1, 10));
    }
}
