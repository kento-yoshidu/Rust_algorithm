// https://atcoder.jp/contests/abc177/tasks/abc177_c

fn run(n: usize, a: Vec<usize>) -> usize {
    let m = 1000000007;

    let mut cum_sum = Vec::new();
    cum_sum.push(a[0]);

    for i in 1..n {
        cum_sum.push((cum_sum[i-1] + a[i]) % m);
    }

    (0..n-1)
        .map(|i| {
            (a[i+1] * cum_sum[i]) % m
        })
        .sum::<usize>() % m
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![1, 2, 3], 11),
            TestCase(4, vec![141421356, 17320508, 22360679, 244949], 437235829),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
