// https://atcoder.jp/contests/abc191/tasks/abc191_b

pub fn run(_n: usize, x: usize, a: Vec<usize>) -> Vec<usize> {
    a.into_iter()
        .filter(|num| {
            *num != x
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![3, 6, 4], run(5, 5, vec![3, 5, 6, 5, 4]));
        assert_eq!(Vec::<usize>::new(), run(3, 3, vec![3, 3, 3]));
    }
}
