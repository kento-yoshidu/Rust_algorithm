// https://atcoder.jp/contests/abc148/tasks/abc148_a

fn run (a: usize, b: usize) -> usize {
    6 - a - b
}

fn run2(a: usize, b: usize) -> usize {
    (1..4)
        .find(|&i| i != a && i != b)
        .unwrap()
}

fn run3(a: usize, b: usize) -> usize {
    a ^ b
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 1, 2),
            TestCase(2, 1, 3),
            TestCase(3, 2, 1),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
