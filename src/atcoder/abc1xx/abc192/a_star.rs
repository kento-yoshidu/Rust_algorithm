// https://atcoder.jp/contests/abc192/tasks/abc192_a

fn run(x: usize) -> usize {
    if x % 100 == 0 {
        100
    } else {
        100 - x % 100
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn ab192_a() {
        let tests = [
            TestCase(140, 60),
            TestCase(1000, 100),
        ];

        for TestCase(x, expected) in tests {
            assert_eq!(run(x), expected);
        }
    }
}
