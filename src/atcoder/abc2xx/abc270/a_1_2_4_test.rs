// https://atcoder.jp/contests/abc270/tasks/abc270_a

pub fn run(a: usize, b: usize) -> usize {
    a | b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(1, 2));
        assert_eq!(7, run(5, 3));
        assert_eq!(0, run(0, 0));
    }
}
