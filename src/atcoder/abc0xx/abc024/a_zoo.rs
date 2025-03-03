// https://atcoder.jp/contests/abc024/tasks/abc024_a

fn run(a: usize, b: usize, c: usize, k: usize, s: usize, t: usize) -> usize {
    if s + t >= k {
        (a*s + b*t) - c*(s+t)
    } else {
        a*s + b*t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(100, 200, 50, 20, 40, 10, 3500),
            TestCase(400, 1000, 400, 21, 10, 10, 14000),
            TestCase(400, 1000, 400, 20, 10, 10, 6000),
        ];

        for TestCase(a, b, c, k, s, t, expected) in tests {
            assert_eq!(run(a, b, c, k, s, t), expected);
        }
    }
}
