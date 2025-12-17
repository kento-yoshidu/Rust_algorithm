// https://atcoder.jp/contests/abc203/tasks/abc203_c

fn run(n: usize, k: usize, ab: Vec<(usize, usize)>) -> usize {
    let mut vec = ab.clone();

    vec.sort_by(|a, b| a.0.cmp(&b.0));

    let mut ans = k;

    for i in 0..n {
        if vec[i].0 > ans {
            break;
        }

        ans += vec[i].1;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, usize);

    #[test]
    fn abc203_c() {
        let tests = [
            TestCase(2, 3, vec![(2, 1), (5, 10)], 4),
            TestCase(5, 1000000000, vec![(1, 1000000000), (2, 1000000000), (3, 1000000000), (4, 1000000000), (5, 1000000000)], 6000000000),
            TestCase(3, 2, vec![(5, 5), (2, 1), (2, 2)], 10),
        ];

        for TestCase(n, k, ab, expected) in tests {
            assert_eq!(run(n, k, ab), expected);
        }
    }
}
