// https://atcoder.jp/contests/abc007/tasks/abc007_1

pub fn run(n: usize) -> usize {
    n - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(4));
        assert_eq!(99, run(100));
        assert_eq!(0, run(1));
    }
}
