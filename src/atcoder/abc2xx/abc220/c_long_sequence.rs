// https://atcoder.jp/contests/abc220/tasks/abc220_c

fn run(n: usize, a: Vec<usize>, x: usize) -> usize {
    let sum: usize = a.iter().sum();

    let mut ans = (x / sum) * n;
    let mut rest = x % sum;

    let mut i = 0;

    loop {
        if rest < a[i] {
            return ans + 1
        } else {
            ans += 1;
            rest -= a[i];
            i = (i + 1) % n;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize, usize);

    #[test]
    fn abc220_c() {
        let tests = [
            TestCase(3, vec![3, 5, 2], 26, 8),
            TestCase(4, vec![12, 34, 56, 78], 1000, 23),
        ];

        for TestCase(n, a, x, expected) in tests {
            assert_eq!(run(n, a, x), expected);
        }
    }
}
