// https://atcoder.jp/contests/abc022/tasks/abc022_a

pub fn run(a: usize) -> usize {
    (a / 2) * (a / 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(25, run(10));
        assert_eq!(900, run(60));
    }
}
