// https://atcoder.jp/contests/abc002/tasks/abc002_1

pub fn run(a: usize, b: usize) -> usize {
    a.max(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(11, run(10, 11));
        assert_eq!(100000000, run(100000000, 10000000));
    }
}
