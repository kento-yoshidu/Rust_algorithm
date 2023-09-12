pub fn run(a: usize, b: usize, c: usize, k: usize, s: usize, t: usize) -> usize {
    if s + t >= k {
        return (a*s + b*t) - c*(s+t);
    } else {
        a*s + b*t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3500, run(100, 200, 50, 20, 40, 10));
        assert_eq!(14000, run(400, 1000, 400, 21, 10, 10));
        assert_eq!(6000, run(400, 1000, 400, 20, 10, 10));
    }
}
