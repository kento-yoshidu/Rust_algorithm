// https://atcoder.jp/contests/abc239/tasks/abc239_b

fn run(x: isize) -> isize {
    if x > 0 {
        x / 10
    } else {
        if x % 10 == 0 {
            x / 10
        } else {
            x / 10 -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize);

    #[test]
    fn abc239_b() {
        let tests = [
            TestCase(47, 4),
            TestCase(-24, -3),
            TestCase(50, 5),
            TestCase(-30, -3),
            TestCase(987654321987654321, 98765432198765432),
        ];

        for TestCase(x, expected) in tests {
            assert_eq!(run(x), expected);
        }
    }
}
