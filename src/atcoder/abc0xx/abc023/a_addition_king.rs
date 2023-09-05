// https://atcoder.jp/contests/abc023/tasks/abc023_a

pub fn run(n: usize) -> usize {
    n / 10 + n % 10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(5, run(23));
        assert_eq!(7, run(70));
        assert_eq!(18, run(99));
    }
}
