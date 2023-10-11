// https://atcoder.jp/contests/abc286/tasks/abc286_a

pub fn run(_n: usize, p: usize, q: usize, r: usize, s: usize, a: Vec<usize>) -> Vec<usize> {
    let mut vec = a.clone();

    (p..=q).zip(r..=s).for_each(|v| {
        vec.swap(v.0-1, v.1-1);
    });

    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![5, 6, 7, 4, 1, 2, 3, 8], run(8, 1, 3, 5, 7, vec![1, 2, 3, 4, 5, 6, 7, 8]));
        assert_eq!(vec![2, 1, 1, 2, 1], run(5, 2 ,3, 4, 5, vec![2, 2, 1, 1, 1]));
        assert_eq!(vec![100, 50], run(2, 1, 1, 2, 2, vec![50, 100]));
        assert_eq!(vec![22, 47, 29, 97, 72, 81, 75, 26, 45, 2], run(10, 2, 4, 7, 9, vec![22, 75, 26, 45, 72, 81, 47, 29, 97, 2]));
    }
}
