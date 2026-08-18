// https://atcoder.jp/contests/awc0023/tasks/awc0023_a

fn run(_n: usize, m: usize, r: usize, t: Vec<usize>) -> usize {
    m * r + t.into_iter().sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<usize>, usize);

    #[test]
    fn awc0023_a() {
        let tests = [
            TestCase(3, 1, 5, vec![10, 20, 30], 65),
            TestCase(5, 2, 10, vec![3, 7, 2, 8, 5], 45),
            TestCase(10, 4, 1000000000, vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000], 4000005500),
        ];

        for TestCase(n, m, r, t, expected) in tests {
            assert_eq!(run(n, m, r, t), expected);
        }
    }
}
