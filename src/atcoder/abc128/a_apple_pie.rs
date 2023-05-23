pub fn run(a: i32, p: i32) -> i32 {
    let piece = p + a * 3;

    piece / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(1, 3));
        assert_eq!(0, run(0, 1));
        assert_eq!(58, run(32, 21));
    }
}
