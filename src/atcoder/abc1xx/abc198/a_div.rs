// https://atcoder.jp/contests/abc198/tasks/abc198_a

pub fn run(n: usize) -> usize {
    n - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, run(2));
        assert_eq!(0, run(1));
        assert_eq!(2, run(3));
    }
}
