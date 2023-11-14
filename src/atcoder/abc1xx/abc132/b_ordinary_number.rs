// https://atcoder.jp/contests/abc132/tasks/abc132_b

pub fn run(_n: usize, p: Vec<usize>) -> usize {
    p.windows(3)
        .filter(|v| {
            (v[0] < v[1] && v[1] < v[2]) || (v[0] > v[1] && v[1] > v[2])
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(5, vec![1, 3, 5, 4, 2]));
        assert_eq!(5, run(9, vec![9, 6, 3, 2, 5, 8, 7, 4, 1]));
    }
}
