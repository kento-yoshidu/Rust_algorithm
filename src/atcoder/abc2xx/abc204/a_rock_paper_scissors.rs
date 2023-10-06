// https://atcoder.jp/contests/abc204/tasks/abc204_a

pub fn run(a: usize, b: usize) -> usize {
    if a == b {
        a
    } else {
        3 - a - b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(0, 1));
        assert_eq!(0, run(0, 0));
    }
}
