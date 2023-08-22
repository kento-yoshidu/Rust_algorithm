// https://atcoder.jp/contests/abc005/tasks/abc005_1

pub fn run(x: usize, y: usize) -> usize {
    y / x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(4, 8));
        assert_eq!(1, run(4, 7));
        assert_eq!(0, run(4, 3));
    }
}
