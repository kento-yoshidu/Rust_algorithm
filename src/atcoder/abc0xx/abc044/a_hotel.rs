// https://atcoder.jp/contests/abc044/tasks/abc044_a

fn run(n: usize, k: usize, x: usize, y: usize) -> usize {
    let first =
        if n <= k {
            n * x
        } else {
            k * x
        };

    let second =
        if n <= k {
            0
        } else {
            (n - k) * y
        };

    first + second
}

fn run2(n: usize, k: usize, x: usize, y: usize) -> usize {
    if n <= k {
        n * x
    } else {
        (k * x) + ((n - k) * y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, 3, 10000, 9000, 48000),
            TestCase(2, 3, 10000, 9000, 20000),
        ];

        for TestCase(n, k, x, y, expected) in tests {
            assert_eq!(run(n, k, x, y), expected);
        }
    }
}
