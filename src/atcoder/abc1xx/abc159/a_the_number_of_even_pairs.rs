// https://atcoder.jp/contests/abc159/tasks/abc159_a

pub fn run(n: isize, m: isize) -> isize {
    n * (n - 1) / 2 + m * (m - 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, run(2, 1));
        assert_eq!(9, run(4, 3));
        assert_eq!(0, run(1, 1));
        assert_eq!(81, run(13, 3));
        assert_eq!(3, run(0, 3));
    }
}
