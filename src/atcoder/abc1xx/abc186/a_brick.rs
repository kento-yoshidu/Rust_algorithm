// https://atcoder.jp/contests/abc186/tasks/abc186_a

#[allow(dead_code)]
pub fn run(n: i32, w: i32) -> i32 {
    (n as f64 / w as f64).floor() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(10, 3));
        assert_eq!(1000, run(1000, 1));
    }
}
