// https://atcoder.jp/contests/abc170/tasks/abc170_a

pub fn run(x: Vec<usize>) -> usize {
    x.iter()
        .enumerate()
        .find(|(_, n)| {
            **n == 0
        })
        .unwrap().0 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, run(vec![0, 2, 3, 4, 5]));
        assert_eq!(2, run(vec![1, 0, 3, 4, 5]));
        assert_eq!(3, run(vec![1, 2, 0, 4, 5]));
        assert_eq!(4, run(vec![1, 2, 3, 0, 5]));
        assert_eq!(5, run(vec![1, 2, 3, 4, 0]));
    }
}
