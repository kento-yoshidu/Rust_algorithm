// https://atcoder.jp/contests/abc376/tasks/abc376_a

pub fn run(_n: usize, c: usize, a: Vec<usize>) -> usize {
    let mut ans = 1;
    let mut cur = a[0];

    for arr in a.windows(2) {
        if arr[1] - cur >= c {
            ans += 1;
            cur = arr[1];
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(6, 5, vec![1, 3, 7, 8, 10, 12], 3),
            TestCase(3, 2, vec![0, 2, 4], 3),
            TestCase(10 , 3, vec![0, 3, 4, 6, 9, 12, 15, 17, 19, 20], 7),
        ];

        for TestCase(n, c, a, expected) in tests {
            assert_eq!(run(n, c, a), expected);
        }
    }
}
