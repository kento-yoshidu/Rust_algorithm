// https://atcoder.jp/contests/abc205/tasks/abc205_a

pub fn run(a: usize, b: usize) -> f64 {
    a as f64 * (b as f64 / 100.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(90.0, run(45, 200));
        assert_eq!(166.5, run(37, 450));
        assert_eq!(0.0, run(0, 1000));
        assert_eq!(0.0, run(50, 0));
    }
}
