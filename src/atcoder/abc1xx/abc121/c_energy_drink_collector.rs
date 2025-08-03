// https://atcoder.jp/contests/abc121/tasks/abc121_c

fn run(_n: usize, m: usize, ab: Vec<(usize, usize)>) -> usize {
    let mut vec = ab.clone();

    vec.sort_by(|a, b| a.0.cmp(&b.0));

    let mut ans = 0;
    let mut count = 0;

    for (a, b) in vec {
        if count >= m {
            return ans;
        }

        if count + b >= m {
            return ans + (m - count) * a;
        } else {
            ans += a*b;
            count += b;
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, usize);

    #[test]
    fn abc121_c() {
        let tests = [
            TestCase(2, 5, vec![(4, 9), (2, 4)], 12),
            TestCase(4, 30, vec![(6, 18), (2, 5), (3, 10), (7, 9)], 130),
            TestCase(1, 100000, vec![(1000000000, 100000)], 100000000000000),
        ];

        for TestCase(n, m, ab, expected) in tests {
            assert_eq!(run(n, m, ab), expected);
        }
    }
}
