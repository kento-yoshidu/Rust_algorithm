// https://atcoder.jp/contests/abc031/tasks/abc031_a

pub fn run(a: usize, d: usize) -> usize {
    a.max(d) * (a.min(d) + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2784, run(31, 87));
        assert_eq!(6666, run(101, 65));
        assert_eq!(12, run(3, 3));
    }
}
