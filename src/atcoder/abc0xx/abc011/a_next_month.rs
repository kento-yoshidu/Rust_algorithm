// https://atcoder.jp/contests/abc011/tasks/abc011_1

pub fn run(n: usize) -> usize {
    n % 12 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(1));
        assert_eq!(1, run(12));
    }
}
