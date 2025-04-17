// https://atcoder.jp/contests/joi2021yo1c/tasks/joi2021_yo1c_c

fn run(_n: usize, _m: usize, a: Vec<usize>, b: Vec<usize>) -> usize {
    a.iter()
        .map(|x| {
            b.iter()
                .filter(move |y| {
                    x <= *y
                })
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, 4, vec![3, 8, 10, 5, 5], vec![1, 5, 4, 9], 8),
            TestCase(3, 5, vec![2000, 2000, 2000], vec![1, 1, 1, 1, 1], 0),
            TestCase(1, 1, vec![1000], vec![1000], 1),
            TestCase(10, 10, vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3], vec![2, 7, 1, 8, 2, 8, 1, 8, 2, 8], 58),
        ];

        for TestCase(n, m, a, b, expected) in tests {
            assert_eq!(run(n, m, a, b), expected);
        }
    }
}