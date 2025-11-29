// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_ai

fn run(n: usize, _q: usize, a: Vec<usize>, lr: Vec<(usize, usize)>) -> Vec<usize> {
    let mut cumsum = vec![0];

    for i in 1..=n {
        cumsum.push(cumsum[i-1]+a[i-1]);
    }

    lr.into_iter()
        .map(|(l, r)| cumsum[r] - cumsum[l-1])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<(usize, usize)>, Vec<usize>);

    #[test]
    fn ma038() {
        let tests = [
            TestCase(10, 5, vec![8, 6, 9, 1, 2, 1, 10, 100, 1000, 10000], vec![(2, 3), (1, 4), (3, 9), (6, 8), (1, 10)], vec![15, 24, 1123, 111, 11137]),
        ];

        for TestCase(n, q, a, lr, expected) in tests {
            assert_eq!(run(n, q, a, lr), expected);
        }
     }
}
