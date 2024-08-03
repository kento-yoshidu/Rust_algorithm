// https://atcoder.jp/contests/agc030/tasks/agc030_a

fn run(a: usize, b: usize, c: usize) -> usize {
    if a + b + 1 <= c {
        b + (a + b) + 1
    } else {
        b + c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 1, 4, 5),
            TestCase(5, 2, 9, 10),
            TestCase(8, 8, 1, 9),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
