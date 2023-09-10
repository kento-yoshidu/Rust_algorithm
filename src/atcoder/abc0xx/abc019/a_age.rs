// https://atcoder.jp/contests/abc019/tasks/abc019_1

use itertools::Itertools;

pub fn run(a: usize, b: usize, c: usize) -> usize {
    *vec![a, b, c].iter().sorted().skip(1).next().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(2, 3, 4));
        assert_eq!(5, run(5, 100, 5));
        assert_eq!(3, run(3, 3, 3));
        assert_eq!(3, run(3, 3, 4));
    }
}
