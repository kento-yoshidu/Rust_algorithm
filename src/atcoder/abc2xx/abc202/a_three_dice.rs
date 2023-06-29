// https://atcoder.jp/contests/abc202/tasks/abc202_a

#[allow(dead_code)]
pub fn run(a: usize, b: usize, c: usize) -> usize {
    21 - (a + b + c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(13, run(1, 4, 3));
        assert_eq!(6, run(5, 6, 4));
    }
}
