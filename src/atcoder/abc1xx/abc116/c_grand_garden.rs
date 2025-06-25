// https://atcoder.jp/contests/abc116/tasks/abc116_c

fn run(n: usize, h: Vec<usize>) -> usize {
    let mut ans = h[0];

    for i in 0..n-1 {
        if h[i] < h[i+1] {
            ans += h[i+1] - h[i];
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn abc116_c() {
        let tests = [
            TestCase(4, vec![1, 2, 2, 1], 2),
            TestCase(5, vec![3, 1, 2, 3, 1], 5),
            TestCase(8, vec![4, 23, 75, 0, 23, 96, 50, 100], 221),
        ];

        for TestCase(n, h, expected) in tests {
            assert_eq!(run(n, h), expected);
        }
    }
}
