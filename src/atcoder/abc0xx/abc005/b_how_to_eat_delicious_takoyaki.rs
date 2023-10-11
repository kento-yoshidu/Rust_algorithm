// https://atcoder.jp/contests/abc005/tasks/abc005_2

fn run(_n: usize, t: Vec<usize>) -> usize {
    *t.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, run(3, vec![1, 2, 3]));
        assert_eq!(3, run(3, vec![3, 3, 3]));
        assert_eq!(1, run(5, vec![3, 1, 4, 1, 5]));
    }
}
