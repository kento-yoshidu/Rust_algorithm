// https://atcoder.jp/contests/abc330/tasks/abc330_a

pub fn run(_n: usize, l: usize, a: Vec<usize>) -> usize {
    a.iter()
        .filter(|&&num| {
            num >= l
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(5, 60, vec![60, 20, 100, 90, 40]));
        assert_eq!(0, run(4, 80, vec![79, 78, 77, 76]));
        assert_eq!(6, run(10, 50, vec![31, 41, 59, 26, 53, 58, 97, 93, 23, 84]));
    }
}
