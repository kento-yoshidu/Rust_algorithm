// https://atcoder.jp/contests/abc008/tasks/abc008_1

#[allow(dead_code)]
pub fn run(s: usize, t: usize) -> usize {
    t - s + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, run(4, 7));
        assert_eq!(1, run(1, 1));
        assert_eq!(2, run(999, 1000));
    }
}
