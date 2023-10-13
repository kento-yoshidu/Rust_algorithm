// https://atcoder.jp/contests/abc262/tasks/abc262_a

pub fn run(y: usize) -> usize {
    (y..).find(|y| y % 4 == 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2022, run(2022));
        assert_eq!(2026, run(2023));
        assert_eq!(3002, run(3000));
    }
}
