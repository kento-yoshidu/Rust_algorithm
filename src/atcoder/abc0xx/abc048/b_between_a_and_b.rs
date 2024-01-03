// https://atcoder.jp/contests/abc048/tasks/abc048_b

pub fn run(a: usize, b: usize, x: usize) -> usize {
    let i = (a + x - 1) / x;
    let j = b / x + 1;

    j - i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(4, 8, 2));
        assert_eq!(6, run(0, 5, 1));
        assert_eq!(0, run(9, 9, 2));
        assert_eq!(333333333333333333, run(1, 1000000000000000000, 3));
    }
}
