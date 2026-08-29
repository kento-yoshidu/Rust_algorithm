// https://atcoder.jp/contests/awc0040/tasks/awc0040_a

fn run(_n: usize, _m: usize, s: usize, p: Vec<usize>, tq: Vec<(usize, usize)>) -> usize {
    let sum: usize = tq.into_iter()
            .map(|(t, q)| {
                let x = p[t-1] * q;
                (x + 1) / 2
            })
            .sum();

    sum + s
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<usize>, Vec<(usize, usize)>, usize);

    #[test]
    fn awc0040_a() {
        let tests = [
            TestCase(3, 2, 100, vec![200, 500, 300], vec![(1, 2), (3, 1)], 450),
            TestCase(5, 4, 1000, vec![150, 300, 99, 500, 1200], vec![(3, 3), (5, 1), (1, 5), (2, 2)], 2424),
            TestCase(6, 5, 0, vec![1, 9999, 5000, 3, 7777, 10000], vec![(2, 10000), (4, 1), (6, 9999), (1, 1), (5, 5000)], 119432503),
        ];

        for TestCase(n, m, s, p, tq, expected) in tests {
            assert_eq!(run(n, m, s, p, tq), expected);
        }
    }
}
