#[allow(dead_code)]
pub fn run(a: i32, b: i32, c: i32, d: i32) -> i32 {
    if a*b == c*d {
        a*b
    } else if a*b > c*d {
        a*b
    } else {
        c*d
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(15, run(3, 5, 2, 7));
        assert_eq!(60000, run(100, 600, 200, 300));
    }
}
