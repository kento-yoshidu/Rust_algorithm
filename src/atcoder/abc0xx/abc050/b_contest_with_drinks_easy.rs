// https://atcoder.jp/contests/abc050/tasks/abc050_b

pub fn run(_n: usize, t: Vec<usize>, _m : usize, p: Vec<(usize, usize)>) -> Vec<usize> {
    let sum: usize = t.iter().sum();

    p.iter().map(|tup| {
        sum - t[tup.0-1] + tup.1
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![6, 9], run(3, vec![2, 1, 4], 2, vec![(1, 1), (2, 3)]));
        assert_eq!(vec![19, 25, 30], run(5, vec![7, 2, 3, 8, 5], 3, vec![(4, 2), (1, 7), (4, 13)]));
    }
}
