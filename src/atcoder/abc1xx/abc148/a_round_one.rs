// https://atcoder.jp/contests/abc148/tasks/abc148_a

pub fn run (a: usize, b: usize) -> usize {
    6 - a - b
}

pub fn run2(a: usize, b: usize) -> usize {
    (1..4).find(|&i| i != a && i != b).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(3, 1));
        assert_eq!(3, run(2, 1));
        assert_eq!(1, run(3, 2));
    }

    #[test]
    fn test2() {
        assert_eq!(2, run2(3, 1));
        assert_eq!(3, run2(2, 1));
        assert_eq!(1, run2(3, 2));
    }
}
