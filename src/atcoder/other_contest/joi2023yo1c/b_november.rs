// https://atcoder.jp/contests/joi2023yo1c/tasks/joi2023_yo1c_b

fn run(a: usize, b: usize) -> usize {
    if a + b * 7 > 30 {
        0
    } else {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(19, 1, 1),
            TestCase(3, 4, 0),
            TestCase(8, 3, 1),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
