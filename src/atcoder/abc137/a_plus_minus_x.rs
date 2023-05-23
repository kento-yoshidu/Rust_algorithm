pub fn run(a: i32, b: i32) -> i32 {
    let vec = vec![a + b, a - b, a * b];

    let max = vec.iter().copied().max().unwrap();

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(-10, run(-13, 3));
        assert_eq!(34, run(1, -33));
        assert_eq!(39, run(13, 3));
    }
}
