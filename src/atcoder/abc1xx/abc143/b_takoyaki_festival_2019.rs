// https://atcoder.jp/contests/abc143/tasks/abc143_b

use itertools::Itertools;

pub fn run(_n: i32, vec: Vec<i32>) -> i32 {
    vec.iter()
        .tuple_combinations()
        .map(|(a, b)| a * b)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(11, run(3, vec![3, 1, 2]));
        assert_eq!(312, run(7, vec![5, 0, 7, 8, 3, 3, 2]));
    }
}
