// https://atcoder.jp/contests/abc178/tasks/abc178_b

use std::cmp::max;

pub fn run(a: isize, b: isize, c: isize, d: isize) -> isize {
    max(max(a*c, a*d), max(b*c, b*d))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(1, 2, 1, 1));
        assert_eq!(-6, run(3, 5, -4, -2));
        assert_eq!(1000000000000000000, run(-1000000000, 0, -1000000000, 0));
    }
}
