// https://atcoder.jp/contests/abc074/tasks/abc074_b

pub fn run(_n: usize, k: usize, x: Vec<usize>) -> usize {
    x.iter()
        .map(|i| {
            (i - 0).min(k - i) * 2
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, run(1, 10, vec![2]));
        assert_eq!(12, run(2, 9, vec![3, 6]));
        assert_eq!(74, run(5, 20, vec![11, 12, 9, 17, 12]));

    }
}
