// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_bz

pub fn run(a: usize, b: usize) -> usize {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(1, 2));
        assert_eq!(100, run(77, 23));
        assert_eq!(200, run(100, 100));
    }
}
