// https://atcoder.jp/contests/abc204/tasks/abc204_b

pub fn run(_n: usize, a: Vec<usize>) -> usize {
    a.iter()
        .map(|i| {
            if 10 < *i {
                i - 10
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(25, run(3, vec![6, 17, 28]));
        assert_eq!(1, run(4, vec![8, 9, 10, 11]));
    }
}
