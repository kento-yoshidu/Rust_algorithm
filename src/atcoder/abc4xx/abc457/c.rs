// https://atcoder.jp/contests/abc457/tasks/abc457_c

pub fn run(_n: usize, k: usize, la: Vec<(usize, Vec<usize>)>, c: Vec<usize>) -> usize {
    let mut sum = 0;

    for (i, r) in c.into_iter().enumerate() {
        let (l, a) = &la[i];

        if sum + l*r >= k {
            return a[(k - sum - 1) % l];
        } else {
            sum += l*r;
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, Vec<usize>)>, Vec<usize>, usize);

    #[test]
    fn abc457_c() {
        let tests = [
            TestCase(3, 9, vec![(3, vec![1, 3, 2]), (1, vec![3]), (2, vec![4, 3])], vec![1, 3, 2], 4),
            TestCase(3, 1, vec![(1, vec![7]), (1, vec![111]), (1, vec![5])], vec![1, 100, 10000], 7),
            TestCase(3, 3163812, vec![(5, vec![1, 2, 3, 4, 5]), (4, vec![9, 8, 7, 6]), (2, vec![10, 11])], vec![87043, 908415, 9814], 9),
        ];

        for TestCase(n, k, la, c, expected) in tests {
            assert_eq!(run(n, k, la, c), expected);
        }
    }
}
