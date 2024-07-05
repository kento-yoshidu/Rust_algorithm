// https://atcoder.jp/contests/abc210/tasks/abc210_a

pub fn run(n: usize, a: usize, x: usize, y: usize) -> usize {
    if n > a {
        (n-a)*y + a*x
    } else {
        x * n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, 3, 20, 15, 90),
            TestCase(10, 10, 100, 1, 1000),
        ];

        for TestCase(n, a, x, y, expected) in tests {
            assert_eq!(run(n, a, x, y), expected);
        }
    }
}
