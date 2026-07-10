// https://atcoder.jp/contests/abc457/tasks/abc457_b

fn run(_n: usize, la: Vec<(usize, Vec<usize>)>, x: usize, y: usize) -> usize {
    la[x-1].1[y-1]
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, Vec<usize>)>, usize, usize, usize);

    #[test]
    fn abc457_b() {
        let tests = [
            TestCase(3, vec![(3, vec![10, 20, 30]), (1, vec![7]), (4, vec![5, 6, 7, 8])], 3, 4, 8),
            TestCase(4, vec![(2, vec![9, 1]), (3, vec![8, 2, 6]), (1, vec![5]), (2, vec![4, 3])], 2, 2, 2),
            TestCase(1, vec![(5, vec![100, 200, 300, 400, 500])], 1, 5, 500),
        ];

        for TestCase(n, la, x, y, expected) in tests {
            assert_eq!(run(n, la, x, y), expected);
        }
    }
}
