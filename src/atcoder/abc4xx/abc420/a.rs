// https://atcoder.jp/contests/abc420/tasks/abc420_a

fn run(x: usize, y: usize) -> usize {
    let m = (x + y) % 12;

    if m != 0 {
        m
    } else {
        12
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn abc420_a() {
        let tests = [
            TestCase(5, 9, 2),
            TestCase(1, 1, 2),
            TestCase(11, 1, 12),
            TestCase(12, 12, 12),
        ];

        for TestCase(x, y, expected) in tests {
            assert_eq!(run(x, y), expected);
        }
    }
}
