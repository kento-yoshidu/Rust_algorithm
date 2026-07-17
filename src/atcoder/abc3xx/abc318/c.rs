// https://atcoder.jp/contests/abc318/tasks/abc318_c

fn run(n: usize, d: usize, p: usize, f: Vec<usize>) -> usize {
    let mut vec = f.clone();
    vec.sort_by(|a, b| b.cmp(a));

    let mut acc = vec![0; n + 1];
    for i in 0..n {
        acc[i + 1] = acc[i] + vec[i];
    }

    // フリーパスを使わずに通常料金で乗った場合
    let mut ans = acc[n];

    let mut i = 1;

    while i * d <= n {
        let covered = i * d;
        let cost = p * i + acc[n] - acc[covered];
        ans = ans.min(cost);
        i += 1;
    }

    let covered = i * d;
    if covered >= n {
        let cost = p * i;
        ans = ans.min(cost);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<usize>, usize);

    #[test]
    fn abc318_c() {
        let tests = [
            TestCase(5, 2, 10, vec![7, 1, 6, 3, 6], 20),
            TestCase(3, 1, 10, vec![1, 2, 3], 6),
            TestCase(8, 3, 1000000000, vec![1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000], 3000000000),
        ];

        for TestCase(n, d, p, f, expected) in tests {
            assert_eq!(run(n, d, p, f), expected);
        }
    }
}
