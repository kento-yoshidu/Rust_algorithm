// https://atcoder.jp/contests/abc178/tasks/abc178_a

fn run(x: usize) -> usize {
    match x {
        0 => 1,
        _ => 0
    }
}

fn run2(x: usize) -> usize {
    x ^ 1
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn abc178_a() {
        let tests = [
            TestCase(1, 0),
            TestCase(0, 1),
        ];

        for TestCase(x, expected) in tests {
            assert_eq!(run(x), expected);
            assert_eq!(run2(x), expected);
        }
    }
}
