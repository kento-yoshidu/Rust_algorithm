
// https://atcoder.jp/contests/abc333/tasks/abc333_c?lang=ja

use itertools::Itertools;

pub fn run(n: usize) -> usize {
    let vec: Vec<usize> =
        (1..=12)
            .map(|i| {
                ("1".to_string().repeat(i)).parse::<usize>().unwrap()
            })
            .collect();

    vec.iter()
        .combinations_with_replacement(3)
        .map(|arr| arr.into_iter().sum::<usize>())
        .sorted()
        .nth(n-1)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(113, run(5));
        assert_eq!(2333, run(19));
        assert_eq!(112222222233, run(333));
    }
}
