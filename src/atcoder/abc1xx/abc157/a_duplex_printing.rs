// https://atcoder.jp/contests/abc157/tasks/abc157_a

fn run(n: usize) -> usize {
    if n % 2 == 0 {
        return  n / 2;
    }

    (n / 2) + 1
}

fn run2(n: usize) -> usize {
    (n as f64 / 2.0).ceil() as usize
}

fn run3(n: usize) -> usize {
    (n + 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn abc157_a() {
        let tests = [
            TestCase(5, 3),
            TestCase(2, 1),
            TestCase(100, 50),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
            assert_eq!(run2(n), expected);
            assert_eq!(run3(n), expected);
        }
    }
}
