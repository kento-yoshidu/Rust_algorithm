// https://atcoder.jp/contests/dwacon5th-prelims/tasks/dwacon5th_prelims_a

fn run(n: usize, a: Vec<usize>) -> usize {
    let avg = a.iter().map(|n| *n as f64).sum::<f64>() / n as f64;

    let mut ans = 0;
    let mut dis = std::f64::MAX;

    for (i, n) in a.into_iter().enumerate() {
        if (n as f64 - avg).abs() < dis {
            ans = i;
            dis = (n as f64 - avg).abs();
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![1, 2, 3], 1),
            TestCase(4, vec![2, 5, 2, 5], 0),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected)
        }
    }
}
