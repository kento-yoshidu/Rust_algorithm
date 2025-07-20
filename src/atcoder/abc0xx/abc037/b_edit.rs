// https://atcoder.jp/contests/abc037/tasks/abc037_b

fn run(n: usize, _q: usize, l: Vec<(usize, usize, usize)>) -> Vec<usize> {
    let mut ans = vec![0; n];

    l.iter()
        .for_each(|(l, r, num)| {
            (*l..=*r).for_each(|i| {
                ans[i-1] = *num;
            })
        });

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize, usize)>, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, 2, vec![(1, 3, 10), (2, 4, 20)], vec![10, 20, 20, 20, 0]),
            TestCase(10, 4, vec![(2, 7, 22), (3, 5, 4), (6, 10, 1), (4, 4, 12)], vec![0, 22, 4, 12, 4, 1, 1, 1, 1, 1]),
        ];

        for TestCase(n, q, l, expected) in tests {
            assert_eq!(run(n, q, l), expected);
        }
    }
}
