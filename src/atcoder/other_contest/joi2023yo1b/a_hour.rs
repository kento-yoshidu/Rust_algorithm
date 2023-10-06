// https://atcoder.jp/contests/joi2023yo1b/tasks/joi2023_yo1b_a

pub fn run(x: usize) -> usize {
    x * 24
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(72, run(3));
        assert_eq!(2400, run(100));
    }
}
