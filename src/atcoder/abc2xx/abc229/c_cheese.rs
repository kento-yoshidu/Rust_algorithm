// https://atcoder.jp/contests/abc229/tasks/abc229_c

fn run(_n: isize, w: isize, ab: Vec<(isize, isize)>) -> isize {
    let mut vec = ab.clone();

    vec.sort_by(|a, b| a.0.cmp(&b.0));
    vec.reverse();

    let mut ans = 0;
    let mut w = w;

    for (a, b) in vec {
        if w <= b {
            return ans + w*a;
        }

        ans += a*b;
        w -= b;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, Vec<(isize, isize)>, isize);

    #[test]
    fn abc229_c() {
        let tests = [
            TestCase(3, 5, vec![(3, 1), (4, 2), (2, 3)], 15),
            TestCase(4, 100, vec![(6, 2), (1, 5), (3, 9), (8, 7)], 100),
            TestCase(10, 3141, vec![(314944731, 649), (140276783, 228), (578012421, 809), (878510647, 519), (925326537, 943), (337666726, 611), (879137070, 306), (87808915, 39), (756059990, 244), (228622672, 291)], 2357689932073),
        ];

        for TestCase(n, w, ab, expected) in tests {
            assert_eq!(run(n, w, ab), expected);
        }
    }
}
