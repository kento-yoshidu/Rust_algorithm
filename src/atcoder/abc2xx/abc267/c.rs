// https://atcoder.jp/contests/abc267/tasks/abc267_c

fn run(n: usize, m: usize, a: Vec<isize>) -> isize {
    let mut c = vec![0; n + 1];
    for i in 0..n {
        c[i + 1] = c[i] + a[i];
    }

    // 重みつき累積和
    let mut d = vec![0; n + 1];
    for i in 0..n {
        d[i + 1] = d[i] + (i as isize + 1) * a[i];
    }

    let mut ans = isize::MIN;

    for k in 0..=n - m {
        let x = d[k + m] - d[k];
        let y = (k as isize) * (c[k + m] - c[k]);

        ans = ans.max(x - y);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<isize>, isize);

    #[test]
    fn abc267_c() {
        let tests = [
            TestCase(4, 2, vec![5, 4, -1, 8], 15),
            TestCase(10, 4, vec![-3, 1, -4, 1, -5, 9, -2, 6, -5, 3], 31),
        ];

        for TestCase(n, m, a, expected) in tests {
            assert_eq!(run(n, m, a), expected);
        }
    }
}
