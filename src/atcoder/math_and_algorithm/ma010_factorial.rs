// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_j

fn calc(ans: usize, n: usize) -> usize {
    if n == 1 {
        ans
    } else {
        calc(ans*n, n-1)
    }
}

fn run(n: usize) -> usize {
    calc(1, n)
}

fn run2(n: usize) -> usize {
    (1..=n)
        .fold(1, |acc, n| {
            acc * n
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(1, 1),
            TestCase(2, 2),
            TestCase(3, 6),
            TestCase(4, 24),
            TestCase(5, 120),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
            assert_eq!(run2(n), expected);
        }
    }
}
