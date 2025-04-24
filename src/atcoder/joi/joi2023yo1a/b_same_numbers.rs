// https://atcoder.jp/contests/joi2023yo1a/tasks/joi2023_yo1a_b

pub fn run(n: usize) -> usize {
    if (n / 10) == (n % 10) {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, run(22));
        assert_eq!(0, run(10));
        assert_eq!(1, run(33));
        assert_eq!(0, run(25));
    }
}
