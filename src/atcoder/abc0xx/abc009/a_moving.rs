// https://atcoder.jp/contests/abc009/tasks/abc009_1

pub fn run(n: usize) -> usize {
    if n % 2 == 0 {
        n / 2
    } else {
        n / 2 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, run(2));
        assert_eq!(3, run(5));
        assert_eq!(1, run(1));
    }
}
