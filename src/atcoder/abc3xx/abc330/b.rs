// https://atcoder.jp/contests/abc330/tasks/abc330_b

fn run(_n: usize, l: usize, r: usize, a: Vec<usize>) -> Vec<usize> {
    a.into_iter()
        .map(|num| {
            if l <= num && num <= r {
                num
            } else if num < l {
                l
            } else {
                r
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<usize>, Vec<usize>);

    #[test]
    fn abc330_b() {
        let tests = [
            TestCase(5, 4, 7, vec![3, 1, 4, 9, 7], vec![4, 4, 4, 7, 7]),
            TestCase(3, 10, 10, vec![11, 10, 9], vec![10, 10, 10]),
        ];

        for TestCase(n, l, r, a, expected) in tests {
            assert_eq!(run(n, l, r, a), expected);
        }
    }
}
