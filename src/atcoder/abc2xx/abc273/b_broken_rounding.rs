// https://atcoder.jp/contests/abc273/tasks/abc273_b

fn calc(n: usize, k: u32) -> usize {
    if n % 10_usize.pow(k) < 5 * 10_usize.pow(k-1) {
        n - (n % 10_usize.pow(k))
    } else {
        (10_usize.pow(k) - n % 10_usize.pow(k)) + n
    }
}

fn run(x: usize, k: u32) -> usize {
    (1..=k)
        .fold(x, |state, k| {
            calc(state, k)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, u32, usize);

    #[test]
    fn abc273_b() {
        let tests = [
            TestCase(2048, 2, 2100),
            TestCase(1, 15, 0),
            TestCase(999, 3, 1000),
            TestCase(314159265358979, 12, 314000000000000),
        ];

        for TestCase(n, k, expected) in tests {
            assert_eq!(run(n, k), expected);
        }
    }
}
