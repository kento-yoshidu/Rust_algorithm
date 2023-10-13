// https://atcoder.jp/contests/abc015/tasks/abc015_2

pub fn run(_n: usize, a: Vec<usize>) -> usize {
    let sum: usize = a.iter().sum();
    let count = a.iter().filter(|n| **n != 0).count();

    (sum as f64 / count as f64).ceil() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, run(4, vec![0, 1, 3, 8]));
        assert_eq!(8, run(5, vec![1, 4, 9, 10, 15]));
    }
}
