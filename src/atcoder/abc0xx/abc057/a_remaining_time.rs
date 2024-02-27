// https://atcoder.jp/contests/abc057/tasks/abc057_a

pub fn run(a: usize, b: usize) -> usize {
    let time = a + b;

    if time >= 24 {
        time - 24
    } else {
        time
    }
}

pub fn run2(a: usize, b: usize) -> usize {
    (a + b) % 24
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(9, 12, 21),
            TestCase(19, 0, 19),
            TestCase(23, 2, 1),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
