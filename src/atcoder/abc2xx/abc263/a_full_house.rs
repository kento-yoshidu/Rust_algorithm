// https://atcoder.jp/contests/abc263/tasks/abc263_a

use std::collections::HashSet;
use itertools::Itertools;

pub fn run(a: usize, b: usize, c: usize, d: usize, e: usize) -> String {
    let vec = vec![a, b, c, d, e];
    let map = vec.iter().counts();
    let set: HashSet<usize> = map.into_values().collect();

    if set == HashSet::from([3, 2]) {
        String::from("Yes")
    } else {
        String::from("No")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Yes"), run(1, 2, 1, 2, 1));
        assert_eq!(String::from("No"), run(12, 12, 11, 1, 2));
    }
}
