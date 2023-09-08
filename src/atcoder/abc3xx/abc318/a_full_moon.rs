// https://atcoder.jp/contests/abc318/tasks/abc318_a

pub fn run(n: isize, m: isize, p: isize) -> isize {
    if n - m < 0 {
        return 0
    }

    (n - m) / p + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(13, 3, 5));
        assert_eq!(0, run(5, 6, 6));
        assert_eq!(628, run(200000, 314, 318));
    }
}
