// https://atcoder.jp/contests/awc0120/tasks/awc0120_a

fn run(_n: usize, k: usize, ms: Vec<(usize, Vec<usize>)>) -> Vec<usize> {
    ms.into_iter()
        .map(|(_, s)| {
            s.into_iter()
                .filter(|s| *s >= k)
                .count()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, Vec<usize>)>, Vec<usize>);

    #[test]
    fn awc0120_a() {
        let tests = [
            TestCase(3, 60, vec![(5, vec![45, 72, 60, 88, 55]), (3, vec![100, 59, 60]), (4, vec![30, 40, 50, 20])], vec![3, 2, 0]),
            TestCase(5, 75, vec![(6, vec![80, 65, 90, 75, 70, 85]), (4, vec![74, 75, 76, 100]), (3, vec![50, 60, 70]), (8, vec![75, 75, 75, 75, 74, 74, 74, 74]), (2, vec![0, 100])], vec![4, 3, 0, 4, 1]),
            TestCase(10, 50, vec![(10, vec![45, 55, 60, 40, 70, 80, 35, 50, 65, 48]), (8, vec![100, 99, 98, 97, 96, 95, 94, 93]), (5, vec![0, 25, 50, 75, 100]), (12, vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 49, 51]), (3, vec![50, 50, 50]), (7, vec![49, 49, 49, 49, 49, 49, 49]), (6, vec![51, 52, 53, 54, 55, 56]), (4, vec![0, 0, 0, 0]), (9, vec![100, 100, 100, 100, 100, 100, 100, 100, 100]), (15, vec![25, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75, 80, 85, 90, 95])], vec![6, 8, 3, 7, 3, 0, 6, 0, 9, 10]),
        ];

        for TestCase(n, k, ms, expected) in tests {
            assert_eq!(run(n, k, ms), expected);
        }
    }
}
