// https://atcoder.jp/contests/abc064/tasks/abc064_b

pub fn run(_n: usize, a: Vec<usize>) -> usize {
    let min = a.iter().min().unwrap();
    let max = a.iter().max().unwrap();

    max - min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(7, run(4, vec![2, 3, 7, 9]));
        assert_eq!(8, run(8, vec![3, 1, 4, 1, 5, 9, 2, 6]));
    }
}
