// https://atcoder.jp/contests/abc139/tasks/abc139_d

pub fn run(n: usize) -> usize {
    (1..n).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, run(2));
        assert_eq!(78, run(13));
        assert_eq!(0, run(1));
    }
}
