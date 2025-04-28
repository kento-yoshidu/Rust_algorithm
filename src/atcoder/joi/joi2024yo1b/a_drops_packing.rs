// https://atcoder.jp/contests/joi2024yo1b/tasks/joi2024_yo1b_a

pub fn run(a: usize, b: usize, c: usize) -> usize {
    a * b + c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(70, run(20, 3, 10));
        assert_eq!(240, run(70, 2, 100));
        assert_eq!(245, run(23, 10, 15));
        assert_eq!(10100, run(100, 100, 100));
    }
}
