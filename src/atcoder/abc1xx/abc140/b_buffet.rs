// https://atcoder.jp/contests/abc140/tasks/abc140_b

pub fn run(n: usize, a: Vec<usize>, b: Vec<usize>, c: Vec<usize>) -> usize {
    (0..n)
        .fold((0, 999),  |(state, prev), i| {
            if prev + 1 == a[i] {
                (state + c[prev-1] + b[a[i]-1], a[i])
            } else {
                (state + b[a[i]-1], a[i])
            }
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(14, run(3, vec![3, 1, 2], vec![2, 5, 4], vec![3, 6]));
        assert_eq!(74, run(4, vec![2, 3, 4, 1], vec![13, 5, 8, 24], vec![45, 9, 15]));
        assert_eq!(150, run(2, vec![1, 2], vec![50, 50], vec![50]));
    }
}
