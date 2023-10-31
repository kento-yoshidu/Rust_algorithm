// https://atcoder.jp/contests/joi2024yo1a/tasks/joi2024_yo1a_d

pub fn run(_n: usize, a: Vec<usize>) -> Vec<usize> {
    let mut vec = a.clone();

    vec.sort();
    vec.dedup();

    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![0, 1, 2, 3, 6, 9], run(8, vec![2, 0, 2, 3, 0, 9, 1, 6]));
        assert_eq!(vec![9], run(3, vec![9, 9, 9]));
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 9], run(10, vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3]));
    }
}
