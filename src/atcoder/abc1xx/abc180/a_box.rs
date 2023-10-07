// https://atcoder.jp/contests/abc179/tasks/abc179_a

pub fn run(n: i32, a: i32, b: i32) -> i32 {
    n - a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(101, run(100, 1, 2));
        assert_eq!(99, run(100, 2, 1));
        assert_eq!(100, run(100, 1, 1));
    }
}
