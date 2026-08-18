// https://atcoder.jp/contests/awc0024/tasks/awc0024_a

fn run(_n: usize, _q: usize, s: Vec<usize>, ab: Vec<(usize, usize)>) -> Vec<&'static str> {
    ab.into_iter()
        .map(|(a, b)| {
            if s[a-1] > s[b-1] {
                "Yes"
            } else {
                "No"
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<(usize, usize)>, Vec<&'static str>);

    #[test]
    fn awc0024_a() {
        let tests = [
            TestCase(3, 4, vec![80, 65, 80], vec![(1, 2), (2, 1), (1, 3), (3, 1)], vec!["Yes", "No", "No", "No"]),
            TestCase(5, 6, vec![72, 85, 60, 85, 90], vec![(5, 2), (2, 4), (1, 3), (4, 2), (3, 5), (2, 1)], vec!["Yes", "No", "Yes", "No", "No", "Yes"]),
            TestCase(10, 8, vec![45, 78, 92, 65, 100, 38, 72, 0, 55, 100], vec![(5, 10), (10, 5), (6, 8), (8, 6), (3, 1), (1, 2), (4, 7), (9, 4)], vec!["No", "No", "Yes", "No", "Yes", "No", "No", "No"]),
        ];

        for TestCase(n, q, s, ab, expected) in tests {
            assert_eq!(run(n, q, s, ab), expected);
        }
    }
}
