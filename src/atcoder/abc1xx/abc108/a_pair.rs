// https://atcoder.jp/contests/abc108/tasks/abc108_a

pub fn run(n: usize) -> usize {
    ((n + 1) / 2) * (n / 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(3));
        assert_eq!(9, run(6));
        assert_eq!(30, run(11));
        assert_eq!(625, run(50));
    }
}
