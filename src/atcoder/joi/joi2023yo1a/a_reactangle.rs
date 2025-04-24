// https://atcoder.jp/contests/joi2023yo1a/tasks/joi2023_yo1a_a

pub fn run(a: usize, b: usize) -> usize {
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(6, run(2, 3));
        assert_eq!(100, run(100, 1));
        assert_eq!(16, run(4, 4));
    }
}
