// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_v

fn run(_n: usize, a: Vec<usize>) -> usize {
    let mut count = vec![0; 100_001];

    for &x in a.iter() {
        count[x] += 1;
    }

    let mut ans = 0usize;

    for i in 1..50_000 {
        let j = 100_000 - i;

        ans += count[i] * count[j];
    }

    ans += count[50_000] * (count[50_000] - 1) / 2;

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn ma022() {
        let tests = [
            TestCase(6, vec![40000, 50000, 20000, 80000, 50000, 30000], 2),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
