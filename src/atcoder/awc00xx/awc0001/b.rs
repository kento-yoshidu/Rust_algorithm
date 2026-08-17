// https://atcoder.jp/contests/awc0001/tasks/awc0001_b

fn run(_n: usize, l: usize, r: usize, p: Vec<usize>) -> isize {
    p.into_iter()
        .enumerate()
        .filter(|(_, num)| l <= *num && *num <= r)
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(i, _)| (i + 1) as isize)
        .unwrap_or(-1)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<usize>, isize);

    #[test]
    fn awc0001_b() {
        let tests = [
            TestCase(5, 60, 80, vec![55, 72, 80, 90, 65], 3),
        ];

        for TestCase(n, l, r, p, expected) in tests {
            assert_eq!(run(n, l, r, p), expected);
        }
    }
}
