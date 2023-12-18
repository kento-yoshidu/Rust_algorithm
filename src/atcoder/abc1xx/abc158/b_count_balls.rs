// https://atcoder.jp/contests/abc158/tasks/abc158_b

pub fn run(n: usize, a: usize, b: usize) -> usize {
    (n / (b + a)) * a + a.min(n % (b + a))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, run(8, 3, 4));
        assert_eq!(0, run(8, 0, 4));
        assert_eq!(2, run(6, 2, 4));
    }
}
