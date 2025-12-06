// https://atcoder.jp/contests/abc183/tasks/abc183_a

fn run(x: isize) -> isize {
    if x < 0 {
        0
    } else {
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize);

    #[test]
    fn abc183_a() {
        let tests = [
            TestCase(1, 1),
            TestCase(0, 0),
            TestCase(-1, 0),
        ];

        for TestCase(x, expected) in tests {
            assert_eq!(run(x), expected);
        }
    }
}
