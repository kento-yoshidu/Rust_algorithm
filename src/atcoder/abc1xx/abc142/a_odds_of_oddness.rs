// https://atcoder.jp/contests/abc142/tasks/abc142_a

pub fn run(a: usize) -> f64 {
    (a as f64 / 2.0).ceil() / a as f64
}

pub fn run2(a: usize) -> f64 {
    (1..=a)
        .filter(|num| num % 2 == 1)
        .count() as f64 / a as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(0.5, run(4));
        assert_eq!(1.0, run(1));
        assert_eq!(0.6, run(5));
    }

    #[test]
    fn test2() {
        assert_eq!(0.5, run2(4));
        assert_eq!(1.0, run2(1));
        assert_eq!(0.6, run2(5));
    }
}
