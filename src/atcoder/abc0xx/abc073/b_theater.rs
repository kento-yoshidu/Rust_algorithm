// https://atcoder.jp/contests/abc073/tasks/abc073_b

pub fn run(n: usize, l: Vec<(usize, usize)>) -> usize {
    l.iter().map(|t| {
        t.1 - t.0
    })
    .sum::<usize>() + n
}

pub fn run2(n: usize, l: Vec<(usize, usize)>) -> usize {
    l.iter().fold(0, |sum, (l, r)| sum + r - l) + n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(7, run(1, vec![(24, 30)]));
        assert_eq!(4, run(2, vec![(6, 8), (3, 3)]));
    }

    #[test]
    fn test2() {
        assert_eq!(7, run2(1, vec![(24, 30)]));
        assert_eq!(4, run2(2, vec![(6, 8), (3, 3)]));
    }
}
