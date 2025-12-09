// https://atcoder.jp/contests/abc189/tasks/abc189_b

fn run(_n: usize, x: usize, vp: Vec<(usize, usize)>) -> isize {
    let mut current = 0.0;

    for (i, (v, p)) in vp.iter().enumerate() {
        current += *v as f64 * (*p as f64 / 100.0);

        if (current > 0.0 && x == 0) || current.round() as usize > x {
            return (i + 1) as isize
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 15, vec![(200, 5), (350, 3)], 2),
            TestCase(2, 10, vec![(200, 5), (350, 3)], 2),
            TestCase(3, 1000000, vec![(1000, 100), (1000, 100), (1000, 100)], -1),
            TestCase(1, 7, vec![(25, 28)], -1),
            TestCase(1, 0, vec![(1, 1)], 1),
        ];

        for TestCase(n, x, vp, expected) in tests {
            assert_eq!(run(n, x, vp), expected);
        }
    }
}
