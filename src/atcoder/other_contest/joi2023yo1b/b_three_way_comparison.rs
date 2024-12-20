// https://atcoder.jp/contests/joi2023yo1b/tasks/joi2023_yo1b_b

fn run(a: usize, b: usize) -> isize {
    if a < b {
        -1
    } else if a == b {
        0
    } else {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 7, -1),
            TestCase(10, 10, 0),
            TestCase(1000, 1, 1),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
