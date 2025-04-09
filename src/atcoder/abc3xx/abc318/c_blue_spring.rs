// https://atcoder.jp/contests/abc318/tasks/abc318_c

fn run(n: usize, d: usize, p: usize, f: Vec<usize>) -> usize {
    let mut vec = f.clone();
    vec.sort_by(|a, b| b.cmp(a));

    // 累積和
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

    #[test]
    fn test() {
        assert_eq!(20, run(5, 2, 10, vec![7, 1, 6, 3, 6]));
        assert_eq!(6, run(3, 1, 10, vec![1, 2, 3]));
        assert_eq!(3000000000, run(8, 3, 1000000000, vec![1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000]));
    }
}
