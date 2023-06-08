pub fn run(a: i32, b: i32, c: i32, d: i32) -> i32 {
    let max = a.max(b);
    let min = c.min(d);

    max - min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(10, run(0, 10, 0, 10));
        assert_eq!(-200, run(-100, -100, 100, 100));
        assert_eq!(200, run(-100, 100, -100, 100));
    }
}
