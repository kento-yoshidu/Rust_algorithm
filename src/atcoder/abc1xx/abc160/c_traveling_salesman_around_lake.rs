// https://atcoder.jp/contests/abc160/tasks/abc160_c

fn run(k: usize, n: usize, a: Vec<usize>) -> usize {
    let mut max = k + a[0] - a.iter().last().unwrap();

    for i in 0..n-1 {
        max = max.max(a[i+1] - a[i]);
    }

    k - max
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(20, 3, vec![5, 10, 15], 10),
            TestCase(20, 3, vec![0, 5, 15], 10),
        ];

        for TestCase(k, n, a, expected) in tests {
            assert_eq!(run(k, n, a), expected);
        }
    }
}
