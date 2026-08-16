// https://atcoder.jp/contests/awc0021/tasks/awc0021_a

fn run(_n: usize, k: usize, ab: Vec<(usize, Vec<usize>)>) -> usize {
    ab.into_iter()
        .map(|(_, b)| {
            b.into_iter()
                .filter(|b| {
                    *b >= k
                })
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, Vec<usize>)>, usize);

    #[test]
    fn awc0021_a() {
        let tests = [
            TestCase(3, 60, vec![(4, vec![45, 72, 60, 88]), (3, vec![59, 60, 61]), (2, vec![100, 30])], 6),
            TestCase(5, 70, vec![(5, vec![65, 70, 80, 55, 90]), (3, vec![69, 71, 70]), (4, vec![100, 85, 72, 68]), (2, vec![50, 49]), (6, vec![75, 80, 85, 90, 95, 100])], 14),
            TestCase(10, 50, vec![(8, vec![23, 45, 67, 89, 12, 34, 56, 78]), (5, vec![50, 50, 50, 49, 51]), (10, vec![0, 10, 20, 30, 40, 50, 60, 70, 80, 90]), (3, vec![100, 100, 100]), (7, vec![45, 46, 47, 48, 49, 50, 51]), (4, vec![25, 75, 25, 75]), (6, vec![99, 98, 97, 96, 95, 94]), (2, vec![0, 100]), (9, vec![55, 55, 55, 55, 55, 44, 44, 44, 44]), (5, vec![1, 2, 3, 4, 5])], 32),
        ];

        for TestCase(n, k, ab, expected) in tests {
            assert_eq!(run(n, k, ab), expected);
        }
    }
}
