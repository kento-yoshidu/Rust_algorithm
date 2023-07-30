// https://atcoder.jp/contests/abc004/tasks/abc004_1

#[allow(dead_code)]
pub fn run(n: usize) -> usize {
    n * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2000, run(1000));
        assert_eq!(2000000, run(1000000));
        assert_eq!(0, run(0));
    }
}
