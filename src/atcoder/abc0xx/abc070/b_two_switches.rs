// https://atcoder.jp/contests/abc070/tasks/abc070_b

use std::cmp::{min, max};

pub fn run(a: isize, b: isize, c: isize, d: isize) -> isize {
    let start = max(a, c);
    let end = min(b, d);

    max(0, end-start)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(50, run(0, 75, 25, 100));
        assert_eq!(0, run(0, 33, 66, 99));
        assert_eq!(60, run(10, 90, 20, 80));
    }
}
