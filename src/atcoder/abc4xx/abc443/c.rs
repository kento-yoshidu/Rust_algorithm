// https://atcoder.jp/contests/abc443/tasks/abc443_c

fn run(_n: usize, t: usize, a: Option<Vec<usize>>) -> usize {
    if a.is_none() {
        return t;
    }

    let mut ans = 0;

    let mut last = 0;

    for a in a.unwrap() {
        if last < a {
            ans += a - last;
            last = a + 100;
        }
    }

    if last < t {
        ans += t - last;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Option<Vec<usize>>, usize);

    #[test]
    fn abc443_c() {
        let tests = [
            TestCase(5, 700, Some(vec![100, 150, 300, 350, 700]), 500),
            TestCase(0, 1000000000, None, 1000000000),
            TestCase(10, 1234, Some(vec![395, 424, 588, 745, 773, 863, 910, 958, 1102, 1195]), 734),
        ];

        for TestCase(n, t, a, expected) in tests {
            assert_eq!(run(n, t, a), expected);
        }
    }
}
