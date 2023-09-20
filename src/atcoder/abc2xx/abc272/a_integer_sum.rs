// https://atcoder.jp/contests/abc272/tasks/abc272_a

pub fn run(_n: usize, vec: Vec<usize>) -> usize {
    vec.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(11, run(3, vec![2, 7, 2]));
        assert_eq!(3, run(1, vec![3]));
    }
}
