// https://atcoder.jp/contests/abc102/tasks/abc102_b

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
        assert_eq!(5, run(4, vec![1, 4, 6, 3]));
        assert_eq!(999999999, run(2, vec![1000000000, 1]));
        assert_eq!(0, run(5, vec![1, 1, 1, 1, 1]));
    }
}
