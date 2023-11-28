// https://atcoder.jp/contests/abc199/tasks/abc199_b

pub fn run(_n: usize, a: Vec<usize>, b: Vec<usize>) -> usize {
    let min = a.iter().min().unwrap();
    let max = b.iter().max().unwrap();

    (*min..=*max)
        .filter(|num| {
            a.iter().zip(b.iter())
                .all(|(l, r)| {
                    l <= num && num <= r
                })
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(2, vec![3, 2], vec![7, 5]));
        assert_eq!(0, run(3, vec![1, 5, 3], vec![10, 7, 3]));
        assert_eq!(2, run(3, vec![3, 2, 5], vec![6, 9, 8]));
    }
}
