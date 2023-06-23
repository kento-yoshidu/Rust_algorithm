#[allow(dead_code)]
pub fn run(n: i32) -> i32 {
    if n % 2 == 0 {
        return  n / 2;
    }

    (n / 2) + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(5));
        assert_eq!(1, run(2));
        assert_eq!(50, run(100));
    }
}
