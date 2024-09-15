// https://atcoder.jp/contests/abc277/tasks/abc277_a

fn run(_n: usize, x: usize, p: Vec<usize>) -> usize {
    p.into_iter()
        .position(|num| num == x)
        .unwrap() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(4, 3, vec![2, 3, 1 ,4]));
        assert_eq!(5, run(5, 2, vec![3, 5, 1, 4, 2]));
        assert_eq!(6, run(6, 6, vec![1, 2, 3, 4, 5, 6]));
    }
}
