// https://atcoder.jp/contests/abc242/tasks/abc242_a

#[allow(dead_code)]
pub fn run(a: usize, b: usize, c: usize, x: usize) -> f64 {
    if x <= a {
        return 1.0
    }

    if b < x {
        return 0.0
    }

    c as f64 / (b - a) as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(0.0425531914893617, run(30, 500, 20, 103));
        assert_eq!(1.0, run(50, 500, 100, 1));
        assert_eq!(0.0, run(1, 2, 1, 1000));
    }
}

