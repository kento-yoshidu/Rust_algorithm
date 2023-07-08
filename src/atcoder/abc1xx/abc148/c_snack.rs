// https://atcoder.jp/contests/abc148/tasks/abc148_c

#[allow(dead_code)]
fn gcd(m: i64, n: i64) -> i64 {
    if n == 0 {
        return m;
    }

    gcd(n, m % n)
}

#[allow(dead_code)]
pub fn run(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test () {
        assert_eq!(6, run(2, 3));
        assert_eq!(18696, run(123, 456));
        assert_eq!(9999900000, run(100000, 99999));
    }
}
