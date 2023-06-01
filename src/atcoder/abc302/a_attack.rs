pub fn run(hp: i64, at: i64) -> i64 {
    if hp % at == 0 {
        hp / at
    } else {
        hp / at + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(7, 3));
        assert_eq!(124999999, run(123456789123456789, 987654321));
        assert_eq!(499999999999999999, run(999999999999999998, 2));
    }
}
