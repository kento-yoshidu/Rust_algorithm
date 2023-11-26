// https://atcoder.jp/contests/abc284/tasks/abc284_b

pub fn run(_n: usize, t: Vec<(usize, Vec<usize>)>) -> Vec<usize> {
    t.iter()
        .map(|v| {
            v.1.iter()
                .filter(|i| **i % 2 == 1)
                .count()
        })
        .collect::<Vec<usize>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![2, 1, 5, 0], run(4, vec![(3, vec![1, 2, 3]), (2, vec![20, 23]), (10, vec![6, 10, 4, 1, 5, 9, 8, 6, 5, 1]), (1, vec![1000000000])]));
    }
}
