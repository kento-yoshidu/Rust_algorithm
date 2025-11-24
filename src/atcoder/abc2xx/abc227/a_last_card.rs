// https://atcoder.jp/contests/abc227/tasks/abc227_a

fn run(n: usize, k: usize, a: usize) -> usize {
    let ans = (k + a - 1) % n;

    if ans != 0 {
        ans
    } else {
        n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn abc227_a() {
        let tests = [
            TestCase(3, 3, 2, 1),
            TestCase(1, 100, 1, 1),
            TestCase(3, 14, 2, 3),
        ];

        for TestCase(n, k, a, expected) in tests {
            assert_eq!(run(n, k, a), expected);
        }
    }
}
