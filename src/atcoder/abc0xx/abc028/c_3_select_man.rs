// https://atcoder.jp/contests/abc028/tasks/abc028_c

use itertools::Itertools;

pub fn run(vec: Vec<usize>) -> usize {
    let mut result: Vec<usize> = vec.iter()
        .permutations(3)
        .map(|t| {
            t[0] + t[1] + t[2]
        })
        .collect();

    result.sort();
    result.dedup();
    result.reverse();

    result[2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(10, run(vec![1, 2, 3, 4, 5]));
        assert_eq!(14, run(vec![1, 2, 3, 5, 8]));
    }
}
