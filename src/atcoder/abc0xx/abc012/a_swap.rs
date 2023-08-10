// https://atcoder.jp/contests/abc012/tasks/abc012_1

pub fn run(a: usize, b: usize) -> Vec<usize> {
    vec![b, a]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![2, 1], run(1, 2));
        assert_eq!(vec![5, 5], run(5, 5));
    }
}
