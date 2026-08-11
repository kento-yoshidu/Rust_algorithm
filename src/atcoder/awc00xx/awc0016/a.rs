// https://atcoder.jp/contests/awc0016/tasks/awc0016_a

fn run(_n: usize, ab: Vec<(usize, usize)>) -> (usize, usize) {
    ab.into_iter()
        .fold((0, 0), |(count, sum), (a, b)| {
            if a > b {
                (count + 1, sum + (a - b))
            } else {
                (count, sum)
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, usize)>, (usize, usize));

    #[test]
    fn awc0016_a() {
        let tests = [
            TestCase(3, vec![(100, 80), (50, 50), (30, 60)], (1, 20)),
        ];

        for TestCase(n, ab, expected) in tests {
            assert_eq!(run(n, ab), expected);
        }
    }
}
