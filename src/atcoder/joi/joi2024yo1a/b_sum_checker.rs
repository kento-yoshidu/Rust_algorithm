// https://atcoder.jp/contests/joi2024yo1a/tasks/joi2024_yo1a_b

use itertools::Itertools;

pub fn run(a: usize, b: usize, c: usize) -> usize {
    let vec = vec![a, b, c];

    if vec.iter().permutations(3).any(|v| {
        v[0] + v[1] == *v[2]
    }) {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, run(3, 5, 2));
        assert_eq!(0, run(32, 3, 4));
        assert_eq!(1, run(100, 50, 50));
        assert_eq!(0, run(1, 1, 1));
    }
}
