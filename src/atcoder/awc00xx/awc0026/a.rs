// https://atcoder.jp/contests/awc0026/tasks/awc0026_a

fn run(_n: usize, k: usize, a: Vec<usize>) -> Vec<usize> {
    a.into_iter()
        .enumerate()
        .filter(|(i, _)| {
            (i + 1) % k == 0
        })
        .map(|(_, a)| a)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<usize>);

    #[test]
    fn awc0026_a() {
        let tests = [
            TestCase(6, 2, vec![10, 20, 30, 40, 50, 60], vec![20, 40, 60]),
            TestCase(10, 3, vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000], vec![300, 600, 900]),
            TestCase(15, 4, vec![1000000000, 999999999, 123456789, 987654321, 111111111, 222222222, 333333333, 444444444, 555555555, 666666666, 777777777, 888888888, 999999998, 100000007, 314159265], vec![987654321, 444444444, 888888888]),
        ];

        for TestCase(n, k, a, expected) in tests {
            assert_eq!(run(n, k, a), expected);
        }
    }
}
