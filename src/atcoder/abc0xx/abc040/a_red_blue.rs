#[allow(dead_code)]
pub fn run(n: i32, x: i32) -> i32 {
    let rest = n - x;

    if rest > 2 {
        x - 1
    } else {
        n - x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, run(5, 2));
        assert_eq!(2, run(6, 4));
        assert_eq!(29, run(90, 30));
    }
}
