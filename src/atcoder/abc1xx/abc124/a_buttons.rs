#[allow(dead_code)]
pub fn run(a: i32, b: i32) -> i32 {
    if a == b {
        return a * 2;
    }

    let large = a.max(b);

    large + large - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(9, run(5, 3));
        assert_eq!(7, run(3, 4));
        assert_eq!(12, run(6, 6));
    }
}
