// https://atcoder.jp/contests/arc009/tasks/arc009_1

fn run(_n: usize, ab: Vec<(usize, usize)>) -> usize {
    let total: usize = ab.iter()
        .map(|(a, b)| a * b)
        .sum();

    (total as f64 * 1.05).floor() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, usize)>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, vec![(4, 120), (2, 130)], 777),
            TestCase(1, vec![(1, 100)], 105),
            TestCase(4, vec![(3, 510), (1, 835), (2, 140), (6, 205)], 4068),
            TestCase(10, vec![(8, 10), (7, 189), (4, 545), (1, 596), (3, 209), (10, 850), (9, 943), (6, 921), (8, 984), (10, 702)], 44321),
        ];

        for TestCase(n, ab, expected) in tests {
            assert_eq!(run(n, ab), expected);
        }
    }
}
