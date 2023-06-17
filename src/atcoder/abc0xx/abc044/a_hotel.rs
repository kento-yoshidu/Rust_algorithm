#[allow(dead_code)]
pub fn run(n: i32, k: i32, x: i32, y: i32) -> i32 {
    let first =
        if n < k {
            n * x
        } else {
            k * x
        };

    let second =
        if n < k {
            0
        } else {
            (n - k) * y
        };

    first + second
}

#[allow(dead_code)]
pub fn run2(n: i32, k: i32, x: i32, y: i32) -> i32 {
    if n < k {
        n * x
    } else {
        (k * x) + ((n - k) * y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(48000, run(5, 3, 10000, 9000));
        assert_eq!(20000, run(2, 3, 10000, 9000));
    }

    #[test]
    fn test2() {
        assert_eq!(48000, run2(5, 3, 10000, 9000));
        assert_eq!(20000, run2(2, 3, 10000, 9000));
    }
}
