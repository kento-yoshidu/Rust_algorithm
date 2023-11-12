// https://atcoder.jp/contests/abc092/tasks/abc092_b

pub fn run(_n: usize, d: usize, x: usize, a: Vec<usize>) -> usize {
    a.iter()
        .map(|i| {
            (d as f64 / *i as f64).ceil() as usize
        })
        .sum::<usize>() + x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(8,  run(3, 7, 1, vec![2, 5, 10]));
        assert_eq!(29, run(2, 8, 20, vec![1, 10]));
        assert_eq!(56, run(5, 30, 44, vec![26, 18, 81, 18, 6]));
    }
}
