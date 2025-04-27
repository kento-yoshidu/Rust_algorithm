// https://atcoder.jp/contests/joi2024yo1a/tasks/joi2024_yo1a_a

pub fn run(x: usize, y: usize) -> usize {
    x + y + 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(9, run(2, 4));
        assert_eq!(48, run(15, 30));
        assert_eq!(3, run(0, 0));
    }
}
