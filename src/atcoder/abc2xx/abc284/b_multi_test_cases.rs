// https://atcoder.jp/contests/abc284/tasks/abc284_b

fn run(_n: usize, t: Vec<(usize, Vec<usize>)>) -> Vec<usize> {
    t.into_iter()
        .map(|v| {
            v.1.iter()
                .filter(|i| **i % 2 == 1)
                .count()
        })
        .collect::<Vec<usize>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, Vec<usize>)>, Vec<usize>);

    #[test]
    fn abc284_b() {
        let tests = [
            TestCase(4, vec![(3, vec![1, 2, 3]), (2, vec![20, 23]), (10, vec![6, 10, 4, 1, 5, 9, 8, 6, 5, 1]), (1, vec![1000000000])], vec![2, 1, 5, 0]),
        ];

        for TestCase(n, t, expected) in tests {
            assert_eq!(run(n, t), expected);
        }
    }
}
