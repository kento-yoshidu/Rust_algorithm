// https://atcoder.jp/contests/abc194/tasks/abc194_a

pub fn run(a: usize, b: usize) -> usize {
    if a + b >= 15 && b >= 8 {
        1
    } else if a + b >= 10 && b >= 3 {
        2
    } else if a + b >= 3 {
        3
    } else {
        4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, run(10, 8));
        assert_eq!(3, run(1, 2));
        assert_eq!(4, run(0, 0));
    }
}
