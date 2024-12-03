// https://atcoder.jp/contests/abc223/tasks/abc223_c

fn run(_n: usize, ab: Vec<(usize, usize)>) -> f64 {
    let vec: Vec<(f64, f64)> = ab.into_iter().map(|(a, b)| (a as f64, b as f64)).collect();

    let mut time = vec.iter()
        .map(|(a, b)| a / b)
        .sum::<f64>() / 2.;

    let mut ans = 0.0;

    for (a, b) in vec {
        if a / b <= time {
            ans += a;
            time -= a / b;
        } else {
            ans += b * time;
            return ans;
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, usize)>, f64);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![(1, 1), (2, 1), (3, 1)], 3.000000000000000),
            TestCase(3, vec![(1, 3), (2, 2), (3, 1)], 3.833333333333333),
            TestCase(5, vec![(3, 9), (1, 2), (4, 6), (1, 5), (5, 3)], 8.916666666666668),
        ];

        for TestCase(n, ab, expected) in tests {
            assert_eq!(run(n, ab), expected);
        }
    }
}
