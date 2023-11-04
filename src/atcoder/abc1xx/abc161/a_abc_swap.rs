// https://atcoder.jp/contests/abc161/tasks/abc161_a

pub fn run(a: usize, b: usize, c: usize) -> Vec<usize> {
    vec![c, a, b]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![3, 1, 2], run(1, 2, 3));
        assert_eq!(vec![100, 100, 100], run(100, 100, 100));
        assert_eq!(vec![31, 41, 59], run(41, 59, 31));
    }
}
