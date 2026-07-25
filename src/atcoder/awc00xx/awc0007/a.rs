// https://atcoder.jp/contests/awc0007/tasks/awc0007_a

fn run(_n: usize, _m: usize, e: Vec<usize>, c: Vec<usize>) -> usize {
    let min = e.into_iter().min().unwrap();

    min * c.into_iter().sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<usize>, usize);

    #[test]
    fn awc0007_a() {
        let tests = [
            TestCase(3, 4, vec![5, 3, 7], vec![2, 4, 1, 3], 30),
            TestCase(5, 6, vec![12, 8, 15, 6, 10], vec![5, 3, 8, 2, 7, 4], 174),
            TestCase(10, 8, vec![100, 250, 80, 320, 150, 90, 200, 180, 75, 110], vec![50, 120, 30, 85, 200, 65, 40, 95], 51375),
        ];

        for TestCase(n, m, e, c, expected) in tests {
            assert_eq!(run(n, m, e, c), expected);
        }
    }
}
