// https://atcoder.jp/contests/abc130/tasks/abc130_a

fn run(x: usize, a: usize) -> usize {
    if x < a {
        0
    } else {
        10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn abc130_a() {
        let tests = [
            TestCase(3, 5, 0),
            TestCase(7, 5, 10),
            TestCase(6, 6, 10),
        ];

        for TestCase(x, a, expected) in tests {
            assert_eq!(run(x, a), expected);
        }
    }
}
