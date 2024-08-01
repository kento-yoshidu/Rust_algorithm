// https://atcoder.jp/contests/tokiomarine2020/tasks/tokiomarine2020_b

fn run(a: isize, v: isize, b: isize, w: isize, t: isize) -> &'static str {
    if v < w {
        return "NO";
    }

    if (b - a).abs() <= (w - v).abs() * t {
        "YES"
    } else {
        "NO"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize, isize, isize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(1, 2, 3, 1, 3, "YES"),
            TestCase(1, 2, 3, 2, 3, "NO"),
            TestCase(1, 2, 3, 3, 3, "NO"),
        ];

        for TestCase(a, v, b, w, t, expected) in tests {
            assert_eq!(run(a, v, b, w, t), expected);
        }
    }
}
