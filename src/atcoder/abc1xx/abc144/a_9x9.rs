// https://atcoder.jp/contests/abc144/tasks/abc144_a

fn run(a: isize, b: isize) -> isize {
    if a > 9 || b > 9 {
        -1
    } else {
        a * b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize);

    #[test]
    fn abc144_a() {
        let tests = [
            TestCase(2, 5, 10),
            TestCase(5, 10, -1),
            TestCase(9, 9, 81),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
