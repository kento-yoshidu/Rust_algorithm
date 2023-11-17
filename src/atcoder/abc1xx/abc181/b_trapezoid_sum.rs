// https://atcoder.jp/contests/abc181/tasks/abc181_b

pub fn run(_n: usize, ab: Vec<(usize, usize)>) -> usize {
    ab.iter()
        .map(|(l, r)| {
            (*l..=*r).sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(18, run(5, vec![(1, 3), (3, 5)]));
        assert_eq!(998244353, run(3, vec![(11, 13), (17, 47), (359, 44683)]));
        assert_eq!(500000500000, run(1, vec![(1, 1000000)]));
    }
}
