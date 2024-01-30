// https://atcoder.jp/contests/abc332/tasks/abc332_a

fn run(_n: usize, s: usize, k: usize, pq: Vec<(usize, usize)>) -> usize {
    let sum = pq.iter()
        .map(|t| {
            t.0 * t.1
        })
        .sum();

    if sum < s {
        sum + k
    } else {
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2100, run(2, 2000, 500, vec![(1000, 1), (100, 6)]));
        assert_eq!(6600, run(3, 2000, 500, vec![(1000, 1), (100, 6), (5000, 1)]));
        assert_eq!(2000, run(2, 2000, 500, vec![(1000, 1), (1000, 1)]));
    }
}
