// https://atcoder.jp/contests/abc142/tasks/abc142_b

pub fn run(_n: usize, k: usize, h: Vec<usize>) -> usize {
    h.iter()
        .filter(|heihgt| **heihgt >= k)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(4, 150, vec![150, 140, 100, 200]));
        assert_eq!(0, run(1, 500, vec![499]));
        assert_eq!(5, run(5, 1, vec![100, 200, 300, 400, 500]));
    }
}
