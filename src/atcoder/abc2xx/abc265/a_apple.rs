// https://atcoder.jp/contests/abc265/tasks/abc265_a

pub fn run(x: usize, y: usize, n: usize) -> usize {
    let tmp = (n / 3) * y;
    let tmp_a = tmp + (n % 3) * x;

    tmp_a.min(x * n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(85, run(10, 25, 10));
        assert_eq!(100, run(10, 40, 10));
        assert_eq!(200, run(100, 100, 2));
        assert_eq!(3400, run(100, 100, 100));
    }
}
