// https://atcoder.jp/contests/abc245/tasks/abc245_b

pub fn run(n: usize, a: Vec<usize>) -> usize {
    (0..=n)
        .find(|i| {
            !a.contains(i)
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, run(8, vec![0, 3, 2, 6, 2, 1, 0, 0]));
        assert_eq!(0, run(3, vec![2000, 2000, 2000]));
    }
}
