// https://atcoder.jp/contests/awc0046/tasks/awc0046_a

fn run(_n: usize, l: isize, d: Vec<isize>) -> isize {
    let mut min = std::isize::MAX;

    for d in d.iter() {
        if *d >= l {
            min = min.min((l - d).abs());
        }
    }

    d.into_iter()
        .enumerate()
        .filter_map(|(i, d)| {
            (l - min == d).then_some(i as isize + 1)
        })
        .min()
        .unwrap_or(-1)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, isize, Vec<isize>, isize);

    #[test]
    fn awc0046_a() {
        let tests = [
            TestCase(5, 10, vec![3, 14, 10, 7, 10], 3),
            TestCase(3, 5, vec![8, 12, 6], -1),
            TestCase(10, 50, vec![45, 60, 50, 30, 50, 70, 48, 55, 49, 50], 3),
            TestCase(20, 1000000000, vec![999999999, 1000000000, 999999998, 500000000, 1000000000, 999999997, 800000000, 1000000000, 999999999, 600000000, 700000000, 900000000, 1000000000, 999999996, 850000000, 1000000000, 750000000, 950000000, 999999999, 1000000000], 2),
            TestCase(1, 1, vec![1], 1),
        ];

        for TestCase(n, l, d, expected) in tests {
            assert_eq!(run(n, l, d), expected);
        }
    }
}
