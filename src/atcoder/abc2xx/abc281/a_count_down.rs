// https://atcoder.jp/contests/abc281/tasks/abc281_a

pub fn run(n: usize) -> Vec<usize> {
    (0..=n).rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![3, 2, 1, 0], run(3));
        assert_eq!(vec![22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0], run(22));
    }
}
