// https://atcoder.jp/contests/abc014/tasks/abc014_1

fn run(a: i32, b: i32) -> i32 {
    if a % b == 0 {
        0
    } else {
        (a / b + 1) * b - a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(i32, i32, i32);

    #[test]
    fn test() {
        let tests = [
            TestCase(7, 3, 2),
            TestCase(5, 5, 0),
            TestCase(1, 100, 99),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
