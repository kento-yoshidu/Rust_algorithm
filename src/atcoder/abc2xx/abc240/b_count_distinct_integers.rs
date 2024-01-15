// https://atcoder.jp/contests/abc240/tasks/abc240_b

pub fn run(_n: usize, a: Vec<usize>) -> usize {
    let mut vec = a.clone();

    vec.sort();
    vec.dedup();

    vec.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(6, vec![1, 4, 1, 2, 2, 1]));
        assert_eq!(1, run(1, vec![1]));
        assert_eq!(7, run(11, vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5]));
    }
}
