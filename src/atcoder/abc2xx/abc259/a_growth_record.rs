// https://atcoder.jp/contests/abc259/tasks/abc259_a

pub fn run(n: usize, m: usize, x: usize, t: usize, d: usize) -> usize {
    if x <= m {
        return t
    }

    t - (n - m) * d
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(168, run(38, 20, 17, 168, 3));
        assert_eq!(1, run(1, 0, 1, 3, 2));
        assert_eq!(90, run(100, 10, 100, 180, 1));
    }
}
