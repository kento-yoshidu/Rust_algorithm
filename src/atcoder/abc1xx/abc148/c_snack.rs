// https://atcoder.jp/contests/abc148/tasks/abc148_c

fn gcd(m: usize, n: usize) -> usize {
    if n == 0 {
        m
    } else {
        gcd(n, m % n)
    }
}

pub fn run(a: usize, b: usize) -> usize {
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
