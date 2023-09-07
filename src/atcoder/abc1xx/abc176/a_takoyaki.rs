// https://atcoder.jp/contests/abc176/tasks/abc176_a

pub fn run(n: i32, x: i32, t: i32) -> i32 {
    if n % x == 0 {
        n / x * t
    } else {
        (n / x) * t + t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(5, run(10, 10, 5));
        assert_eq!(10, run(20, 10, 5));
        assert_eq!(12, run(20, 12, 6));
        assert_eq!(1000000, run(1000, 1, 1000));
    }
}
