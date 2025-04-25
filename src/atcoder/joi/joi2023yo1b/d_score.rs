// https://atcoder.jp/contests/joi2023yo1b/tasks/joi2023_yo1b_d

fn run(_n: usize, a: Vec<usize>, _m: usize, b: Vec<usize>) -> usize {
    a.iter()
        .fold(0, |mut score, num| {
            score += num;

            if b.contains(&score) {
                0
            } else {
                score
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, vec![3, 1, 4, 1], 4, vec![2, 7, 1, 8], 0),
            TestCase(5, vec![1, 4, 1, 4, 2], 3, vec![1, 3, 5], 6),
            TestCase(2, vec![10, 10], 3, vec![1, 11, 11], 20),
        ];

        for TestCase(n, a, m, b, expected) in tests {
            assert_eq!(run(n, a, m, b), expected);
        }
    }
}
