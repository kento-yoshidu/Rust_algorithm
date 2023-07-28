// https://atcoder.jp/contests/abc137/tasks/abc137_a

#[allow(dead_code)]
pub fn run(a: i32, b: i32) -> i32 {
    *[a+b, a-b, a*b].iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(-10, run(-13, 3));
        assert_eq!(34, run(1, -33));
        assert_eq!(39, run(13, 3));
    }
}
