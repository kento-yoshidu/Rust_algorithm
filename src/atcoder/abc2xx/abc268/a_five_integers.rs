// https://atcoder.jp/contests/abc268/tasks/abc268_a

pub fn run(v: Vec<usize>) -> usize {
    let mut vec = v.clone();

    vec.sort();
    vec.dedup();

    vec.len()
}

pub fn run2(v: Vec<usize>) -> usize {
    use itertools::Itertools;

    v.iter().unique().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(vec![31, 9, 24, 31, 24]));
        assert_eq!(1, run(vec![0, 0, 0, 0, 0]));
        assert_eq!(5, run(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn test2() {
        assert_eq!(3, run2(vec![31, 9, 24, 31, 24]));
        assert_eq!(1, run2(vec![0, 0, 0, 0, 0]));
        assert_eq!(5, run2(vec![1, 2, 3, 4, 5]));
    }
}
