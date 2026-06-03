// https://cses.fi/problemset/task/1745

fn run(n: usize, x: Vec<usize>) -> (usize, Vec<usize>) {
    let len: usize = x.iter().sum();
    let mut dp = vec![false; len + 1];

    dp[0] = true;

    for i in 0..n {
        for j in (x[i]..=len).rev() {
            dp[j] |= dp[j - x[i]];
        }
    }

    let t: Vec<usize> = dp
        .clone()
        .into_iter()
        .skip(1)
        .enumerate()
        .filter(|(_, b)| *b)
        .map(|(i, _)| i+1)
        .collect();

    (t.len(), t)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, (usize, Vec<usize>));

    #[test]
    fn cses_1633() {
        let tests = [
            TestCase(4, vec![4, 2, 5, 2], (9, vec![2, 4, 5, 6, 7, 8, 9, 11, 13])),
        ];

        for TestCase(n, x, expected) in tests {
            assert_eq!(run(n, x), expected);
        }
    }
}
