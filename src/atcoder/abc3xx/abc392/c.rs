// https://atcoder.jp/contests/abc392/tasks/abc392_c

fn run(n: usize, p: Vec<usize>, q: Vec<usize>) -> Vec<usize> {
    let mut q_inv = vec![0; n+1];

    for (i, &v) in q.iter().enumerate() {
        q_inv[v] = i;
    }

    (1..=n)
        .map(|i| q[p[q_inv[i]] - 1])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<usize>, Vec<usize>);

    #[test]
    fn abc392_c() {
        let tests = [
            TestCase(4, vec![4, 3, 2, 1], vec![2, 3, 1, 4], vec![3, 4, 1, 2]),
            TestCase(10, vec![2, 6, 4, 3, 7, 8, 9, 10, 1, 5], vec![1, 4, 8, 2, 10, 5, 7, 3, 9, 6], vec![4, 8, 6, 5, 3, 10, 9, 2, 1, 7]),
        ];

        for TestCase(n, p, q, expected) in tests {
            assert_eq!(run(n, p, q), expected);
        }
    }
}
