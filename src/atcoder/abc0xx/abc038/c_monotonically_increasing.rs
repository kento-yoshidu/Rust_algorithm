// https://atcoder.jp/contests/abc038/tasks/abc038_c

fn run(n: usize, a: Vec<usize>) -> usize {
    let mut ans = 0;
    let mut r = 0;

    for l in 0..n {
        while r < n && (r <= l || a[r] > a[r-1]) {
            r += 1;
        }

        ans += r - l;

        if l == r {
            r += 1;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCaes(usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCaes(5, vec![1, 2, 3, 2, 1], 8),
            TestCaes(4, vec![1, 2, 3, 4], 10),
            TestCaes(6, vec![3, 3, 4, 1, 2, 2], 8),
            TestCaes(6, vec![1, 5, 2, 3, 4, 2], 10),
        ];

        for TestCaes(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
