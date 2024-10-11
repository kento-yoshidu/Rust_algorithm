// https://atcoder.jp/contests/abc037/tasks/abc037_c

fn run(n: usize, k: usize, a: Vec<usize>) -> usize {
    let mut cum = vec![0];

    for i in 1..=n {
        cum.push(cum[i-1] + a[i-1]);
    }

    (0..n-k+1)
        .map(|i| {
            cum[i+k] - cum[i]
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, 3, vec![1, 2, 4, 8, 16], 49),
            TestCase(20, 10, vec![100000000, 100000000, 98667799, 100000000, 100000000, 100000000, 100000000, 99986657, 100000000, 100000000, 100000000, 100000000, 100000000, 98995577, 100000000, 100000000, 99999876, 100000000, 100000000, 99999999], 10988865195),
        ];

        for TestCase(n, k, a, expected) in tests {
            assert_eq!(run(n, k, a), expected);
        }
    }
}
