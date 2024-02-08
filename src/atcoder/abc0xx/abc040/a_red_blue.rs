// https://atcoder.jp/contests/abc040/tasks/abc040_a

use std::cmp::min;

pub fn run(n: i32, x: i32) -> i32 {
    min(n - x, x - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, run(5, 2));
        assert_eq!(2, run(6, 4));
        assert_eq!(29, run(90, 30));
    }
}
