// https://atcoder.jp/contests/joi2024yo1b/tasks/joi2024_yo1b_b

pub fn run(x: usize) -> usize {
    if x % 7 == 2 {
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
        assert_eq!(1, run(2));
        assert_eq!(0, run(10));
        assert_eq!(1, run(100));
    }
}
