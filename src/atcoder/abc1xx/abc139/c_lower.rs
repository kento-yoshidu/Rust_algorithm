// https://atcoder.jp/contests/abc139/tasks/abc139_c

fn run(n: usize, h: Vec<usize>) -> usize {
    (1..n)
        .fold((0, 0), |(max_h, current), i| {
            if h[i-1] >= h[i] {
                if i == n-1 {
                    (max_h.max(current+1), current)
                } else {
                    (max_h, current+1)
                }
            } else {
                (max_h.max(current), 0)
            }
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn abc139_c() {
        let tests = [
            TestCase(5, vec![10, 4, 8, 7, 3], 2),
            TestCase(7, vec![4, 4, 5, 6, 6, 5, 5], 3),
            TestCase(4, vec![1, 2, 3, 4], 0),
            TestCase(4, vec![1, 1, 1, 1], 3),
            TestCase(4, vec![1, 1, 1, 1, 2], 3),
            TestCase(4, vec![2, 1, 3, 4, 5], 1),
        ];

        for TestCase(n, h, expected) in tests {
            assert_eq!(run(n, h), expected);
        }
    }
}
