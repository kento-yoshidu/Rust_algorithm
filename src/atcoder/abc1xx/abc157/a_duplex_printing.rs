// https://atcoder.jp/contests/abc157/tasks/abc157_a

pub fn run(n: i32) -> i32 {
    if n % 2 == 0 {
        return  n / 2;
    }

    (n / 2) + 1
}

pub fn run2(n: usize) -> usize {
    (n as f64 / 2.0).ceil() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(5));
        assert_eq!(1, run(2));
        assert_eq!(50, run(100));
    }

    #[test]
    fn test2() {
        assert_eq!(3, run2(5));
        assert_eq!(1, run2(2));
        assert_eq!(50, run2(100));
    }
}
