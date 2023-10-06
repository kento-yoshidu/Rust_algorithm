//https://atcoder.jp/contests/abc209/tasks/abc209_a

pub fn run(a: usize, b: usize) -> usize {
    (a..=b).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(2, 4));
        assert_eq!(91, run(10, 100));
        assert_eq!(0, run(3, 2));
    }
}
