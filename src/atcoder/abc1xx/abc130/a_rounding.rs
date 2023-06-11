#[allow(dead_code)]
pub fn run(x: i32, a: i32) -> i32 {
    if x < a {
        0
    } else {
        10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(0, run(3, 5));
        assert_eq!(10, run(7, 5));
        assert_eq!(10, run(6, 6));
    }
}
