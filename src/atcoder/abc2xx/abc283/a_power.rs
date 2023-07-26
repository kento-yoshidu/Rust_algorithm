// https://atcoder.jp/contests/abc283/tasks/abc283_a

#[allow(dead_code)]
pub fn run(a: usize, b: usize) -> usize {
    a.pow(b as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(64, run(4, 3));
        assert_eq!(3125, run(5, 5));
        assert_eq!(8, run(8, 1));
    }
}
