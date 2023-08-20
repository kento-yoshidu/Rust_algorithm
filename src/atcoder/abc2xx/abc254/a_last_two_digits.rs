// https://atcoder.jp/contests/abc254/tasks/abc254_a

pub fn run(n: usize) -> usize {
    n - (100 * (n / 100))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(54, run(254));
        assert_eq!(01, run(101));
    }
}
