// https://atcoder.jp/contests/abc332/tasks/abc332_a

fn run(_n: usize, s: usize, k: usize, pq: Vec<(usize, usize)>) -> usize {
    let sum = pq.into_iter()
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

    struct TestCase(usize, usize, usize, Vec<(usize, usize)>, usize);

    #[test]
    fn abc332_a() {
        let tests = [
            TestCase(2, 2000, 500, vec![(1000, 1), (100, 6)], 2100),
            TestCase(3, 2000, 500, vec![(1000, 1), (100, 6), (5000, 1)], 6600),
            TestCase(2, 2000, 500, vec![(1000, 1), (1000, 1)], 2000),
        ];

        for TestCase(n, s, k, pq, expected) in tests {
            assert_eq!(run(n, s, k, pq), expected);
        }
    }
}
