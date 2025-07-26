// https://atcoder.jp/contests/abc409/tasks/abc409_c

fn run(n: usize, l: usize, d: Vec<usize>) -> usize {
    if l % 3 != 0 {
        return 0;
    }

    let mut acc: Vec<usize> = vec![0; l];

    let mut x = 0;

    for i in 0..n {
        if i != 0 {
            x += d[i-1]
        }

        x %= l;
        acc[x] += 1;
    }

    let mut ans = 0;

    for i in 0..l/3 {
        ans += acc[i] * acc[i+l/3] * acc[i+2*l/3];
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn abc409_c() {
        let tests = [
            TestCase(5, 6, vec![4, 3, 1, 2], 2),
            TestCase(4, 4, vec![1, 1, 1], 0),
            TestCase(10, 12, vec![4, 4, 5, 7, 1, 7, 0, 8, 5], 13),
        ];

        for TestCase(n, l, d, expected) in tests {
            assert_eq!(run(n, l, d), expected);
        }
    }
}
